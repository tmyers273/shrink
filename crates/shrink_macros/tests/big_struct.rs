#![allow(dead_code)]

use chrono::{DateTime, NaiveDate, Utc};
use shrink_macros::{Classify, ClassifyEnum};

#[derive(Classify)]
struct BigStruct {
    cid: u64,
    enum1: Enum1,
    pid: Option<u64>,
    string: String,
    date1: Option<NaiveDate>,
    date2: Option<NaiveDate>,
    datetime1: DateTime<Utc>,
    datetime2: DateTime<Utc>,
    enum2: Enum2,
    enum3: Enum3,
    complex: Option<Vec<Enum4>>,
    opt_string: Option<String>,
    struct1: Child5,
    struct_opt1: Option<Child2>,
    struct_opt2: Option<Child8>,
    struct_opt3: Option<Enum11>,
}

#[derive(ClassifyEnum)]
enum Enum1 {
    A,
    B,
    C,
}

#[derive(ClassifyEnum)]
enum Enum2 {
    A,
    B,
    C,
    D,
}

#[derive(ClassifyEnum)]
enum Enum3 {
    A,
    B,
    C,
    D,
}

#[derive(ClassifyEnum)]
enum Enum4 {
    A,
    B,
    C,
    D,
    E,
}

#[derive(Classify)]
struct Child2 {
    enum5: Enum5,
    opt_vec_struct3: Option<Vec<Child3>>,
    opt_vec_struct4: Option<Vec<Child4>>,
}

#[derive(ClassifyEnum)]
enum Enum5 {
    A,
    B,
    C,
    D,
    E,
}

#[derive(Classify)]
struct Child3 {
    placement: Enum6,
    percentage: i64,
}

#[derive(ClassifyEnum)]
enum Enum6 {
    A,
    B,
    C,
    D,
    E,
}

#[derive(Classify)]
struct Child4 {
    a: Enum7,
    b: i64,
}

#[derive(ClassifyEnum)]
enum Enum7 {
    A,
}

#[derive(Classify)]
struct Child5 {
    a: Enum8,
    b: Enum9,
    c: Child6,
}

#[derive(ClassifyEnum)]
enum Enum8 {
    A,
}

#[derive(ClassifyEnum)]
enum Enum9 {
    A,
    B,
    C,
}

#[derive(Classify)]
struct Child6 {
    a: Child7,
}

#[derive(Classify)]
struct Child7 {
    a: Enum10,
    b: Option<f64>,
    c: Option<f64>,
}

#[derive(ClassifyEnum)]
enum Enum10 {
    A,
    B,
    C,
}

#[derive(ClassifyEnum)]
enum Child8 {
    A,
    B,
    C,
    D,
}

#[derive(ClassifyEnum)]
enum Enum11 {
    A,
    B,
}

#[test]
fn it_works() {
    // Just a compile time check to make sure the proc macro works
}
