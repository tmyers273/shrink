use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

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
