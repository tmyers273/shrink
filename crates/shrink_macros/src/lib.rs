use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type};

#[proc_macro_derive(Classify)]
pub fn derive_classify(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let classify_body = generate_classify_body(&input.data);

    let expanded = quote! {
        impl #impl_generics Classify for #name #ty_generics #where_clause {
            type Output = u64;

            fn classify(&self) -> Self::Output {
                use std::hash::{Hash, Hasher};
                use std::collections::hash_map::DefaultHasher;

                let mut hasher = DefaultHasher::new();
                #classify_body
                hasher.finish()
            }
        }
    };

    TokenStream::from(expanded)
}

fn generate_classify_body(data: &Data) -> proc_macro2::TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote! {
                        self.#name.classify().hash(&mut hasher);
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, _)| {
                    let index = syn::Index::from(i);
                    quote! {
                        self.#index.classify().hash(&mut hasher);
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unit => quote! {},
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

#[proc_macro_derive(ClassifyEnum)]
pub fn classify_enum_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let output_type = quote! { u64 };

    let classify_match_arms = match &input.data {
        Data::Enum(enum_data) => enum_data
            .variants
            .iter()
            .enumerate()
            .map(|(index, variant)| {
                let variant_name = &variant.ident;
                match &variant.fields {
                    Fields::Unit => {
                        quote! {
                            #name::#variant_name => #index as u64
                        }
                    }
                    Fields::Unnamed(fields) => {
                        let field_types: Vec<&Type> =
                            fields.unnamed.iter().map(|f| &f.ty).collect();
                        let field_count = field_types.len();
                        let field_names: Vec<syn::Ident> = (0..field_count)
                            .map(|i| {
                                syn::Ident::new(&format!("f{}", i), proc_macro2::Span::call_site())
                            })
                            .collect();

                        quote! {
                            #name::#variant_name(#(#field_names),*) => {
                                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                                (#index as u64).hash(&mut hasher);
                                #(
                                    #field_names.classify().hash(&mut hasher);
                                )*
                                std::hash::Hasher::finish(&hasher)
                            }
                        }
                    }
                    Fields::Named(fields) => {
                        let field_names: Vec<syn::Ident> = fields
                            .named
                            .iter()
                            .enumerate()
                            .map(|(i, f)| {
                                f.ident
                                    .clone()
                                    .unwrap_or_else(|| format_ident!("field{}", i))
                            })
                            .collect();

                        quote! {
                            #name::#variant_name { #(#field_names),* } => {
                                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                                (#index as u64).hash(&mut hasher);
                                #(
                                    #field_names.classify().hash(&mut hasher);
                                )*
                                std::hash::Hasher::finish(&hasher)
                            }
                        }
                    }
                }
            }),
        _ => {
            return quote! {
                compile_error!("ClassifyEnum can only be derived for enums");
            }
            .into()
        }
    };

    let expanded = quote! {
        impl shrink::Classify for #name {
            type Output = #output_type;

            fn classify(&self) -> Self::Output {
                use std::hash::{Hash, Hasher};
                use std::collections::hash_map::DefaultHasher;
                match self {
                    #(#classify_match_arms,)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
