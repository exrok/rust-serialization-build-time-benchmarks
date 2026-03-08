#![allow(unused_imports, dead_code)]
use jsony_macros::Jsony;
#[derive(Jsony)]
#[jsony(Json)]
struct S0 {
    f0: Option<String>,
    f1: Vec<u32>,
    f2: Vec<u32>,
    f3: f32,
    f4: f32,
    f5: u64,
    f6: bool,
    f7: u8,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S1 {
    f0: bool,
    f1: u8,
    f2: String,
    f3: S0,
    f4: bool,
    f5: i32,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S2 {
    f0: f32,
    f1: u64,
    f2: bool,
    f3: S0,
    f4: Option<String>,
    f5: u64,
    f6: i32,
    f7: Vec<u32>,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S3 {
    f0: S1,
    f1: S0,
    f2: S1,
    f3: String,
    f4: S2,
    f5: Option<String>,
    f6: u8,
    f7: i8,
    f8: f32,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S4 {
    f0: S2,
    f1: String,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S5 {
    f0: f32,
    f1: S1,
    f2: S1,
    f3: S2,
    f4: f32,
    f5: u64,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S6 {
    f0: Vec<u32>,
    f1: f32,
    f2: S0,
    f3: S3,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S7 {
    f0: Vec<u32>,
    f1: f32,
    f2: f32,
    f3: S1,
    f4: Option<String>,
    f5: S2,
    f6: S0,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S8 {
    f0: Option<String>,
    f1: Option<String>,
    f2: Vec<u32>,
    f3: u8,
    f4: bool,
    f5: i32,
    f6: S4,
    f7: S7,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S9 {
    f0: f32,
    f1: S2,
    f2: i8,
    f3: S7,
    f4: S7,
    f5: Vec<u32>,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S10 {
    f0: S1,
    f1: f32,
    f2: S0,
    f3: S1,
    f4: u8,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S11 {
    f0: S5,
    f1: u8,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S12 {
    f0: S10,
    f1: String,
    f2: Vec<u32>,
    f3: S5,
    f4: S10,
    f5: u64,
    f6: String,
    f7: bool,
    f8: i32,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S13 {
    f0: S2,
    f1: S10,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S14 {
    f0: S3,
    f1: S7,
    f2: Vec<u32>,
    f3: S5,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S15 {
    f0: S5,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S16 {
    f0: Option<String>,
    f1: Option<String>,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S17 {
    f0: String,
    f1: S7,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S18 {
    f0: bool,
    f1: S2,
    f2: S2,
    f3: i32,
    f4: Option<String>,
    f5: bool,
    f6: S0,
    f7: Option<String>,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S19 {
    f0: S4,
    f1: S7,
    f2: Vec<u32>,
    f3: f32,
    f4: i8,
    f5: i32,
    f6: S16,
    f7: S5,
    f8: S1,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S20 {
    f0: String,
    f1: i32,
    f2: String,
    f3: String,
    f4: u64,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S21 {
    f0: Vec<u32>,
    f1: S2,
    f2: S5,
    f3: S7,
    f4: S1,
    f5: S20,
    f6: S1,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S22 {
    f0: S1,
    f1: S1,
    f2: u8,
    f3: S4,
    f4: S7,
    f5: S18,
    f6: String,
    f7: S0,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S23 {
    f0: S10,
    f1: Vec<u32>,
    f2: S2,
    f3: S1,
    f4: f32,
    f5: S3,
    f6: S3,
    f7: S16,
    f8: S3,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S24 {
    f0: S3,
    f1: u8,
    f2: Vec<u32>,
    f3: i8,
    f4: S16,
    f5: S7,
    f6: S7,
    f7: S3,
    f8: S7,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S25 {
    f0: S1,
    f1: i32,
    f2: S10,
    f3: S4,
    f4: i32,
    f5: i8,
    f6: S2,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S26 {
    f0: S20,
    f1: S1,
    f2: S2,
    f3: S7,
    f4: i32,
    f5: Option<String>,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S27 {
    f0: S2,
    f1: bool,
    f2: u64,
    f3: String,
    f4: S0,
    f5: S20,
    f6: S5,
    f7: bool,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S28 {
    f0: S5,
    f1: S7,
    f2: S18,
    f3: String,
    f4: S5,
    f5: S18,
    f6: i32,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S29 {
    f0: S3,
    f1: S1,
    f2: S0,
    f3: bool,
    f4: S18,
    f5: S18,
    f6: S10,
    f7: Vec<u32>,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S30 {
    f0: S20,
    f1: bool,
    f2: S3,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S31 {
    f0: S3,
    f1: S2,
    f2: i8,
    f3: S16,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S32 {
    f0: bool,
    f1: S4,
    f2: i32,
    f3: S5,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S33 {
    f0: S16,
    f1: S4,
    f2: S16,
    f3: S18,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S34 {
    f0: S7,
    f1: Vec<u32>,
    f2: S20,
    f3: S0,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S35 {
    f0: S3,
    f1: S3,
    f2: S20,
    f3: i8,
    f4: Vec<u32>,
    f5: bool,
    f6: S3,
    f7: i8,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S36 {
    f0: Vec<u32>,
    f1: S2,
    f2: S18,
    f3: S20,
    f4: S3,
    f5: S20,
    f6: bool,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S37 {
    f0: S0,
    f1: String,
    f2: Vec<u32>,
    f3: S2,
    f4: u8,
    f5: S18,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S38 {
    f0: S3,
    f1: S5,
    f2: S0,
    f3: i32,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S39 {
    f0: S5,
    f1: S2,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S40 {
    f0: bool,
    f1: S4,
    f2: S20,
    f3: S7,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S41 {
    f0: i8,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S42 {
    f0: S41,
    f1: Option<String>,
    f2: S20,
    f3: S7,
    f4: S3,
    f5: S4,
    f6: String,
    f7: S20,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S43 {
    f0: String,
    f1: bool,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S44 {
    f0: S2,
    f1: S41,
    f2: S16,
    f3: S4,
    f4: S41,
    f5: S41,
    f6: String,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S45 {
    f0: String,
    f1: String,
    f2: S7,
    f3: S20,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S46 {
    f0: S4,
    f1: S1,
    f2: f32,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S47 {
    f0: S43,
    f1: String,
    f2: f32,
    f3: i32,
    f4: S20,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S48 {
    f0: S5,
    f1: S2,
    f2: S41,
    f3: S4,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S49 {
    f0: S10,
    f1: S20,
    f2: S18,
    f3: u8,
    f4: S18,
    f5: i8,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S50 {
    f0: S2,
    f1: Option<String>,
    f2: Vec<u32>,
    f3: Vec<u32>,
    f4: bool,
    f5: S18,
    f6: S47,
    f7: S18,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S51 {
    f0: S1,
    f1: S41,
    f2: S10,
    f3: S47,
    f4: u64,
    f5: u64,
    f6: S4,
    f7: f32,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S52 {
    f0: S5,
    f1: S3,
    f2: Option<String>,
    f3: S47,
    f4: S3,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S53 {
    f0: S18,
    f1: String,
    f2: S0,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S54 {
    f0: bool,
    f1: f32,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S55 {
    f0: Vec<u32>,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S56 {
    f0: S55,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S57 {
    f0: S18,
    f1: S4,
    f2: S20,
    f3: Option<String>,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S58 {
    f0: S18,
    f1: S0,
    f2: S2,
    f3: i32,
    f4: S56,
    f5: u8,
    f6: f32,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S59 {
    f0: S47,
    f1: S18,
    f2: S20,
    f3: S16,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S60 {
    f0: S4,
    f1: S43,
    f2: S7,
    f3: S1,
    f4: bool,
    f5: i8,
    f6: S2,
    f7: u64,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S61 {
    f0: S18,
    f1: f32,
    f2: S56,
    f3: f32,
    f4: S16,
    f5: S3,
    f6: i32,
    f7: bool,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S62 {
    f0: S18,
    f1: S5,
    f2: S0,
    f3: S16,
    f4: u8,
    f5: S54,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S63 {
    f0: S3,
    f1: String,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S64 {
    f0: S18,
    f1: S55,
    f2: S1,
    f3: S55,
    f4: String,
    f5: String,
    f6: f32,
    f7: S54,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S65 {
    f0: S47,
    f1: S20,
    f2: i32,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S66 {
    f0: S54,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S67 {
    f0: S66,
    f1: S47,
    f2: S10,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S68 {
    f0: i32,
    f1: S41,
    f2: i32,
    f3: S55,
    f4: i8,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S69 {
    f0: u8,
    f1: S41,
    f2: S47,
    f3: S66,
    f4: Vec<u32>,
    f5: S68,
    f6: bool,
    f7: Vec<u32>,
    f8: S66,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S70 {
    f0: S54,
    f1: u8,
    f2: f32,
    f3: S0,
    f4: Option<String>,
    f5: S43,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S71 {
    f0: S56,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S72 {
    f0: S20,
    f1: S71,
    f2: S65,
    f3: S0,
    f4: f32,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S73 {
    f0: S10,
    f1: S43,
}

#[derive(Jsony)]
#[jsony(Json)]
struct S74 {
    f0: i8,
    f1: S47,
    f2: u8,
    f3: S1,
}
