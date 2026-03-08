use std::borrow::Cow;

use proc_macro2::{Delimiter, Group, Ident, Literal, TokenStream, TokenTree};

use crate::features::{FieldAnnotator, FieldFeatureDecl};
use crate::schema::{
    Enum, ItemKind, Struct, FLAGS, IMPL_FROM_BINARY, IMPL_FROM_JSON, IMPL_TO_BINARY, IMPL_TO_JSON,
};

#[derive(Clone)]
pub enum Libary {
    Jsony { path: Option<String> },
    Merde,
    Sonic,
    Nanoserde,
    Musli,
    Serde,
    Miniserde,
    Midiserde,
    Baseline,
    Facet,
}

impl Libary {
    pub fn default() -> Vec<Libary> {
        vec![
            Libary::Jsony { path: None },
            Libary::Miniserde,
            Libary::Nanoserde,
            Libary::Serde,
        ]
    }

    pub fn all() -> Vec<Libary> {
        vec![
            Libary::Jsony { path: None },
            Libary::Miniserde,
            Libary::Nanoserde,
            Libary::Serde,
            Libary::Merde,
            Libary::Musli,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Libary::Jsony { path: Some(..) } => "jsony(local)",
            Libary::Jsony { .. } => "jsony",
            Libary::Merde => "merde",
            Libary::Facet => "facet",
            Libary::Sonic => "sonic-rs",
            Libary::Nanoserde => "nanoserde",
            Libary::Musli => "musli",
            Libary::Serde => "serde",
            Libary::Miniserde => "miniserde",
            Libary::Midiserde => "midiserde",
            Libary::Baseline => "baseline",
        }
    }

    pub fn crate_prefix(&self) -> &'static str {
        match self {
            Libary::Jsony { path: Some(_) } => "jsony_local",
            Libary::Jsony { .. } => "jsony",
            Libary::Merde => "merde",
            Libary::Facet => "facet",
            Libary::Sonic => "sonic",
            Libary::Nanoserde => "nanoserde",
            Libary::Musli => "musli",
            Libary::Serde => "serde",
            Libary::Miniserde => "miniserde",
            Libary::Midiserde => "midiserde",
            Libary::Baseline => "baseline",
        }
    }

    pub fn supports_social(&self) -> bool {
        !matches!(
            self,
            Libary::Merde | Libary::Miniserde | Libary::Midiserde | Libary::Baseline
        )
    }

    pub fn supports_multi_format(&self) -> bool {
        matches!(
            self,
            Libary::Jsony { .. } | Libary::Nanoserde | Libary::Serde | Libary::Musli
        )
    }

    pub fn dependencies(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Libary::Jsony { path: Some(path) } => {
                return Cow::Owned(format!(
                    r#"
                jsony = {{path = "{path}/jsony", default-features = false}}
                jsony_macros = {{path = "{path}/jsony_macros", default-features = false}}

                [profile.dev.package."jsony"]
                incremental = false
                [profile.dev.package."jsony_macros"]
                incremental = false
                "#
                ))
            }
            Libary::Baseline => "",
            Libary::Facet => {
                r#"
facet = "0.44"
facet-json = "0.44"
            "#
            }
            Libary::Jsony { .. } => {
                r#"
                jsony = {version = "=0.1.9", default-features = false}
                jsony_macros = {version = "=0.1.8", default-features = false}
            "#
            }
            Libary::Merde => {
                r#"merde = { version = "10", features = ["core", "json", "deserialize"] }"#
            }
            Libary::Nanoserde => "nanoserde = \"0.2\"",
            Libary::Musli => "musli = { version = \"0.0.149\", features = [\"json\"]}",
            Libary::Serde => {
                r#"
            serde_derive = "1"
            serde = "1"
            serde_json = "1"
            "#
            }
            Libary::Sonic => "sonic-rs = \"0.3.14\"\nserde = \"1\"",
            Libary::Miniserde => "miniserde = \"0.1\"",
            Libary::Midiserde => "midiserde = \"0.1\"\nminiserde = \"0.1\"",
        })
    }

    pub fn dependencies_multi(&self) -> Cow<'static, str> {
        match self {
            Libary::Serde => Cow::Borrowed(
                r#"
            serde_derive = "1"
            serde = "1"
            serde_json = "1"
            bincode = "1"
            "#,
            ),
            Libary::Musli => {
                Cow::Borrowed(r#"musli = { version = "0.0.149", features = ["json", "storage"]}"#)
            }
            _ => self.dependencies(),
        }
    }

    pub fn gen_to_json_string(&self, out: &mut Vec<TokenTree>, func: &dyn Fn(&mut Vec<TokenTree>)) {
        match self {
            Libary::Jsony { .. } => splat!(out; jsony::to_json([func(out)])),
            Libary::Nanoserde => splat!(out; ([func(out)]).serialize_json()),
            Libary::Serde => splat!(out; serde_json::to_string([func(out)]).unwrap()),
            Libary::Sonic => splat!(out; sonic_rs::to_string([func(out)]).unwrap()),
            Libary::Miniserde => splat!(out; miniserde::json::to_string([func(out)])),
            Libary::Midiserde => splat!(out; midiserde::json::to_string([func(out)])),
            Libary::Merde => splat!(out; merde::json::to_string([func(out)]).unwrap()),
            Libary::Musli => splat!(out; musli::json::to_string([func(out)]).unwrap()),
            Libary::Baseline => splat!(out; {let _ = [func(out)]; String::new()}),
            Libary::Facet => splat!(out; facet_json::to_string([func(out)]).unwrap()),
        }
    }

    pub fn gen_from_json_str(
        &self,
        out: &mut Vec<TokenTree>,
        ty: &Ident,
        func: &dyn Fn(&mut Vec<TokenTree>),
    ) {
        match self {
            Libary::Jsony { .. } => splat!(out; jsony::from_json::<[#ty]>([func(out)])),
            Libary::Nanoserde => splat!(out; [#ty]::deserialize_json([func(out)])),
            Libary::Serde => splat!(out; serde_json::from_str::<[#ty]>([func(out)])),
            Libary::Sonic => splat!(out; sonic_rs::from_str::<[#ty]>([func(out)])),
            Libary::Miniserde => {
                splat!(out; miniserde::json::from_str::<[#ty]>([func(out)]))
            }
            Libary::Midiserde => {
                splat!(out; midiserde::json::from_str::<[#ty]>([func(out)]))
            }
            Libary::Merde => splat!(out; merde::json::from_str::<[#ty]>([func(out)])),
            Libary::Musli => splat!(out; musli::json::from_str::<[#ty]>([func(out)])),
            Libary::Baseline => splat!(out; Err::<[#ty], &str>([func(out)])),
            Libary::Facet => splat!(out; facet_json::from_str::<[#ty]>([func(out)])),
        }
    }

    pub fn gen_prelude(&self, out: &mut Vec<TokenTree>) {
        splat!(out; #![[allow(unused_imports, dead_code)]]);
        self.gen_imports(out);
    }

    pub fn gen_module_prelude(&self, out: &mut Vec<TokenTree>) {
        self.gen_imports(out);
    }

    pub fn gen_module_prelude_multi(&self, out: &mut Vec<TokenTree>) {
        self.gen_imports_multi(out);
    }

    fn gen_imports_multi(&self, out: &mut Vec<TokenTree>) {
        match self {
            Libary::Jsony { .. } => splat!(out; use jsony_macros::Jsony;),
            Libary::Nanoserde => splat!(out; use nanoserde::{DeBin, SerBin, DeJson, SerJson};),
            Libary::Serde => splat!(out; use serde_derive::{Deserialize, Serialize};),
            Libary::Musli => splat!(out; use musli::{Encode, Decode};),
            other => panic!("gen_imports_multi not supported for {}", other.name()),
        }
    }

    fn gen_imports(&self, out: &mut Vec<TokenTree>) {
        match self {
            Libary::Jsony { .. } => splat!(out; use jsony_macros::Jsony;),
            Libary::Merde => splat!(out; ),
            Libary::Nanoserde => splat!(out; use nanoserde::{DeJson, SerJson};),
            Libary::Musli => splat!(out; use musli::{Encode, Decode};),
            Libary::Serde => splat!(out; use serde_derive::{Deserialize, Serialize};),
            Libary::Sonic => splat!(out; use sonic_rs::{Deserialize, Serialize};),
            Libary::Miniserde => splat!(out; use miniserde::{Serialize, Deserialize};),
            Libary::Midiserde => splat!(out; use midiserde::{Serialize, Deserialize};),
            Libary::Facet => splat!(out; use facet::Facet;),
            Libary::Baseline => (),
        }
    }

    pub fn gen_compat_json(&self, out: &mut Vec<TokenTree>) {
        match self {
            Libary::Jsony { .. } => splat! { out;
                pub fn to_json(value: &impl jsony::ToJson) -> String { jsony::to_json(value) }
                pub fn from_json<T: for<~a> jsony::FromJson<~a> >(s: &str) -> T { jsony::from_json::<T>(s).unwrap() }
            },
            Libary::Serde => splat! { out;
                pub fn to_json(value: &impl serde::Serialize) -> String { serde_json::to_string(value).unwrap() }
                pub fn from_json<~a, T: serde::de::DeserializeOwned>(s: &str) -> T { serde_json::from_str::<T>(s).unwrap() }
            },
            Libary::Sonic => splat! { out;
                pub fn to_json(value: &impl serde::Serialize) -> String { sonic_rs::to_string(value).unwrap() }
                pub fn from_json<T: serde::de::DeserializeOwned>(s: &str) -> T { sonic_rs::from_str::<T>(s).unwrap() }
            },
            Libary::Nanoserde => splat! { out;
                pub fn to_json(value: &impl nanoserde::SerJson) -> String { value.serialize_json() }
                pub fn from_json<T: nanoserde::DeJson>(s: &str) -> T { T::deserialize_json(s).unwrap() }
            },
            Libary::Musli => splat! { out;
                pub fn to_json<T: musli::en::Encode<musli::mode::Text> >(value: &T) -> String { musli::json::to_string(value).unwrap() }
                pub fn from_json<T: for<~a> musli::de::Decode<~a, musli::mode::Text, musli::alloc::Global> >(s: &str) -> T { musli::json::from_str::<T>(s).unwrap() }
            },
            Libary::Miniserde => splat! { out;
                pub fn to_json(value: &impl miniserde::Serialize) -> String { miniserde::json::to_string(value) }
                pub fn from_json<T: miniserde::Deserialize>(s: &str) -> T { miniserde::json::from_str::<T>(s).unwrap() }
            },
            Libary::Midiserde => splat! { out;
                pub fn to_json(value: &impl midiserde::Serialize) -> String { midiserde::json::to_string(value) }
                pub fn from_json<T: midiserde::Deserialize>(s: &str) -> T { midiserde::json::from_str::<T>(s).unwrap() }
            },
            Libary::Baseline => splat! { out;
                pub fn to_json<T>(_value: &T) -> String { String::new() }
                pub fn from_json<T>(_s: &str) -> T { unimplemented!("baseline does not support deserialization") }
            },
            Libary::Facet => splat! { out;
                pub fn to_json<~a>(value: &(impl facet::Facet<~a> + ~a)) -> String { facet_json::to_string(value).unwrap() }
                pub fn from_json<T: for<~a> facet::Facet<~a> >(s: &str) -> T { facet_json::from_str::<T>(s).unwrap() }
            },
            Libary::Merde => {
                panic!("gen_compat_json not supported for {:?}", self.name())
            }
        }
    }

    pub fn gen_compat_binary(&self, out: &mut Vec<TokenTree>) {
        match self {
            Libary::Jsony { .. } => splat! { out;
                pub fn to_binary(value: &impl jsony::ToBinary) -> Vec<u8> { jsony::to_binary(value) }
                pub fn from_binary<T: for<~a> jsony::FromBinary<~a> >(bytes: &[[u8]]) -> T { jsony::from_binary::<T>(bytes).unwrap() }
            },
            Libary::Nanoserde => splat! { out;
                pub fn to_binary(value: &impl nanoserde::SerBin) -> Vec<u8> { nanoserde::SerBin::serialize_bin(value) }
                pub fn from_binary<T: nanoserde::DeBin>(bytes: &[[u8]]) -> T { T::deserialize_bin(bytes).unwrap() }
            },
            Libary::Serde => splat! { out;
                pub fn to_binary(value: &impl serde::Serialize) -> Vec<u8> { bincode::serialize(value).unwrap() }
                pub fn from_binary<T: serde::de::DeserializeOwned>(bytes: &[[u8]]) -> T { bincode::deserialize(bytes).unwrap() }
            },
            Libary::Musli => splat! { out;
                pub fn to_binary<T: musli::en::Encode<musli::mode::Binary> >(value: &T) -> Vec<u8> { musli::storage::to_vec(value).unwrap() }
                pub fn from_binary<T: for<~a> musli::de::Decode<~a, musli::mode::Binary, musli::alloc::Global> >(bytes: &[[u8]]) -> T { musli::storage::from_slice(bytes).unwrap() }
            },
            other => panic!("gen_compat_binary not supported for {}", other.name()),
        }
    }

    pub fn compat_module_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        self.gen_compat_json(&mut out);
        crate::token::to_rust(out.into_iter().collect())
    }

    pub fn compat_module_multi_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        self.gen_compat_json(&mut out);
        self.gen_compat_binary(&mut out);
        crate::token::to_rust(out.into_iter().collect())
    }
}

fn emit_jsony_flags(out: &mut Vec<TokenTree>, flags: FLAGS) {
    let start = out.len();
    let to_json = flags & IMPL_TO_JSON != 0;
    let from_json = flags & IMPL_FROM_JSON != 0;
    let to_binary = flags & IMPL_TO_BINARY != 0;
    let from_binary = flags & IMPL_FROM_BINARY != 0;
    if to_binary && from_binary {
        splat!(out; Binary);
    } else {
        if from_binary {
            splat!(out; FromBinary);
        }
        if to_binary {
            if out.len() != start {
                splat!(out; ,);
            }
            splat!(out; ToBinary);
        }
    }
    if to_json && from_json {
        if out.len() != start {
            splat!(out; ,);
        }
        splat!(out; Json);
    } else {
        if from_json {
            if out.len() != start {
                splat!(out; ,);
            }
            splat!(out; FromJson);
        }
        if to_json {
            if out.len() != start {
                splat!(out; ,);
            }
            splat!(out; ToJson);
        }
    }
    let inner = TokenStream::from_iter(out.drain(start..));
    out.push(TokenTree::Group(Group::new(Delimiter::Parenthesis, inner)));
}

fn emit_serde_derive(out: &mut Vec<TokenTree>, flags: FLAGS) {
    let ser = flags & (IMPL_TO_JSON | IMPL_TO_BINARY) != 0;
    let de = flags & (IMPL_FROM_JSON | IMPL_FROM_BINARY) != 0;
    let start = out.len();
    if de {
        splat!(out; Deserialize);
    }
    if ser {
        if out.len() != start {
            splat!(out; ,);
        }
        splat!(out; Serialize);
    }
    let inner = TokenStream::from_iter(out.drain(start..));
    out.push(TokenTree::Group(Group::new(Delimiter::Parenthesis, inner)));
}

fn emit_nanoserde_derive(out: &mut Vec<TokenTree>, flags: FLAGS) {
    let start = out.len();
    if flags & IMPL_FROM_BINARY != 0 {
        splat!(out; DeBin);
    }
    if flags & IMPL_TO_BINARY != 0 {
        if out.len() != start {
            splat!(out; ,);
        }
        splat!(out; SerBin);
    }
    if flags & IMPL_FROM_JSON != 0 {
        if out.len() != start {
            splat!(out; ,);
        }
        splat!(out; DeJson);
    }
    if flags & IMPL_TO_JSON != 0 {
        if out.len() != start {
            splat!(out; ,);
        }
        splat!(out; SerJson);
    }
    let inner = TokenStream::from_iter(out.drain(start..));
    out.push(TokenTree::Group(Group::new(Delimiter::Parenthesis, inner)));
}

fn emit_musli_derive(out: &mut Vec<TokenTree>, flags: FLAGS) {
    let encode = flags & (IMPL_TO_JSON | IMPL_TO_BINARY) != 0;
    let decode = flags & (IMPL_FROM_JSON | IMPL_FROM_BINARY) != 0;
    let start = out.len();
    if decode {
        splat!(out; Decode);
    }
    if encode {
        if out.len() != start {
            splat!(out; ,);
        }
        splat!(out; Encode);
    }
    let inner = TokenStream::from_iter(out.drain(start..));
    out.push(TokenTree::Group(Group::new(Delimiter::Parenthesis, inner)));
}

impl<'a> Struct<'a> {
    pub fn generate_def(&self, out: &mut Vec<TokenTree>, lib: &Libary) {
        self.generate_def_flagged(out, lib, false, self.flags)
    }

    pub fn generate_def_public(&self, out: &mut Vec<TokenTree>, lib: &Libary) {
        self.generate_def_flagged(out, lib, true, self.flags)
    }

    pub fn generate_def_with_flags(
        &self,
        out: &mut Vec<TokenTree>,
        lib: &Libary,
        public: bool,
        flags: FLAGS,
    ) {
        self.generate_def_flagged(out, lib, public, flags)
    }

    fn generate_def_flagged(
        &self,
        out: &mut Vec<TokenTree>,
        lib: &Libary,
        public: bool,
        flags: FLAGS,
    ) {
        match lib {
            Libary::Jsony { .. } => self.generate_def_jsony(out, public, flags),
            Libary::Merde => self.generate_def_merde(out),
            Libary::Nanoserde => self.generate_def_nanoserde(out, public, flags),
            Libary::Musli => self.generate_def_musli(out, public, flags),
            Libary::Serde | Libary::Sonic | Libary::Miniserde | Libary::Midiserde => {
                self.generate_def_serde(out, public, flags)
            }
            Libary::Facet => self.generate_def_facet(out, public),
            Libary::Baseline => self.generate_def_baseline(out, public),
        }
    }

    fn generate_def_naked(&self, out: &mut Vec<TokenTree>) {
        if self.kind == ItemKind::Tuple {
            splat! {
                out;
                struct [#self.name] (
                    [for (field in self.fields) {
                        [field.ty().gen(out)],
                    }]
                );
            }
            return;
        }
        splat! {
            out;
            struct [#self.name] [?(self.lifetimes !=0) <~a>] {
                [for (field in self.fields) {
                    [#field.name]: [field.ty().gen(out)],
                }]
            }
        }
    }

    fn generate_def_nanoserde(&self, out: &mut Vec<TokenTree>, public: bool, flags: FLAGS) {
        if self.kind == ItemKind::Tuple {
            splat! {
                out;
                #[[derive [emit_nanoserde_derive(out, flags)] ]]
                #[[nserde(transparent)]]
                [?(public) pub] struct [#self.name] (
                    [for (field in self.fields) {
                        [?(public) pub] [field.ty().gen(out)],
                    }]
                );
            }
            return;
        }
        splat! {
            out;
            #[[derive [emit_nanoserde_derive(out, flags)] ]]
            [?(public) pub] struct [#self.name] [?(self.lifetimes !=0) <~a>] {
                [for (field in self.fields) {
                    [if let Some(rename) = field.rename {
                        splat!(out; #[[ nserde(rename = [#Literal::string(rename)]) ]]);
                    }]
                    [?(public) pub] [#field.name]: [field.ty().gen(out)],
                }]
            }
        }
    }

    fn generate_def_merde(&self, out: &mut Vec<TokenTree>) {
        if self.kind == ItemKind::Tuple {
            splat! {
                out;
                struct [#self.name] (
                    [for (field in self.fields) {
                        [field.ty().gen(out)],
                    }]
                );
            }
            return;
        }
        let mut first = true;
        splat! {
            out;
            struct [#self.name] [?(self.lifetimes !=0) <~a>] {
                [for (field in self.fields) {
                    [#field.name]: [field.ty().gen(out)],
                }]
            }

            merde::derive! {
                impl (Serialize, Deserialize) for struct [#self.name] [?(self.lifetimes !=0) <~a>] {
                    [for field in self.fields {
                        if first {
                            first = false;
                        } else {
                            splat!(out; ,);
                        }
                        splat!(out; [#field.name]);
                    }]
                }
            }
        }
    }

    fn generate_def_baseline(&self, out: &mut Vec<TokenTree>, public: bool) {
        if self.kind == ItemKind::Tuple {
            splat! {
                out;
                [?(public) pub] struct [#self.name] (
                    [for (field in self.fields) {
                        [?(public) pub] [field.ty().gen(out)],
                    }]
                );
            }
            return;
        }
        splat! {
            out;
            [?(public) pub] struct [#self.name] [?(self.lifetimes !=0) <~a>] {
                [for (field in self.fields) {
                    [?(public) pub] [#field.name]: [field.ty().gen(out)],
                }]
            }
        }
    }

    fn generate_def_jsony(&self, out: &mut Vec<TokenTree>, public: bool, flags: FLAGS) {
        if self.kind == ItemKind::Tuple {
            splat! {
                out;
                #[[ derive(Jsony) ]]
                #[[ jsony [emit_jsony_flags(out, flags)] ]]
                [?(public) pub] struct [#self.name] (
                    [for (field in self.fields) {
                        [?(public) pub] [field.ty().gen(out)],
                    }]
                );
            }
            return;
        }
        splat! {
            out;
            #[[ derive(Jsony) ]]
            #[[ jsony [emit_jsony_flags(out, flags)] ]]
            [?(public) pub] struct [#self.name] [?(self.lifetimes !=0) <~a>] {
                [for (field in self.fields) {
                    [if let Some(rename) = field.rename {
                        splat!(out; #[[ jsony(rename = [#Literal::string(rename)]) ]]);
                    }]
                    [?(public) pub] [#field.name]: [field.ty().gen(out)],
                }]
            }
        }
    }

    pub fn generate_adv_def_jsony(&'a self, out: &mut Vec<TokenTree>) {
        let mut fields = FieldAnnotator::of_struct(self);
        splat! {
            out;
            #[[ derive(Jsony) ]]
            #[[ jsony [emit_jsony_flags(out, self.flags)] ]]
            struct [#self.name] [?(self.lifetimes !=0) <~a>] {
                [while let Some((field, mut features)) = fields.next() {
                    let attr_start = out.len();
                    while let Some(feature) = features.next() {
                        if attr_start != out.len() {
                            splat!(out; ,)
                        }
                        match feature {
                            FieldFeatureDecl::Skip => splat!(out; skip),
                            FieldFeatureDecl::TrivalDefault => splat!(out; default),
                            FieldFeatureDecl::Default{seed} => {
                                splat!(out; default = [field.ty().generate_random_default(out, seed)])
                            },
                            FieldFeatureDecl::Rename(value) => {
                                splat!(out; rename = [#Literal::string(&value)]);
                            },
                        }
                    }
                    if attr_start != out.len() {
                        let inner = TokenTree::Group(Group::new(Delimiter::Parenthesis,
                            TokenStream::from_iter(out.drain(attr_start..))
                        ));
                        splat!(out; #[[ jsony [@inner] ]])
                    }

                    splat!(out; [#field.name]: [field.ty().gen(out)],)
                }]
            }
        }
    }

    fn generate_def_facet(&self, out: &mut Vec<TokenTree>, public: bool) {
        if self.kind == ItemKind::Tuple {
            splat! {
                out;
                #[[ derive(Facet) ]]
                #[[ facet(transparent) ]]
                [?(public) pub] struct [#self.name] (
                    [for (field in self.fields) {
                        [?(public) pub] [field.ty().gen(out)],
                    }]
                );
            }
            return;
        }
        splat! {
            out;
            #[[ derive(Facet) ]]
            [?(public) pub] struct [#self.name] [?(self.lifetimes !=0) <~a>] {
                [for (field in self.fields) {
                    [if let Some(rename) = field.rename {
                        splat!(out; #[[ facet(rename = [#Literal::string(rename)]) ]]);
                    }]
                    [?(public) pub] [#field.name]: [field.ty().gen(out)],
                }]
            }
        }
    }

    fn generate_def_musli(&self, out: &mut Vec<TokenTree>, public: bool, flags: FLAGS) {
        if self.kind == ItemKind::Tuple {
            splat! {
                out;
                #[[ derive [emit_musli_derive(out, flags)] ]]
                #[[ musli(transparent) ]]
                [?(public) pub] struct [#self.name] (
                    [for (field in self.fields) {
                        [?(public) pub] [field.ty().gen(out)],
                    }]
                );
            }
            return;
        }
        splat! {
            out;
            #[[ derive [emit_musli_derive(out, flags)] ]]
            [?(public) pub] struct [#self.name] [?(self.lifetimes !=0) <~a>] {
                [for (field in self.fields) {
                    [?(field.ty().lifetimes() !=0) #[[serde(borrow)]]]
                    [if let Some(rename) = field.rename {
                        splat!(out; #[[ musli(Text, name = [#Literal::string(rename)]) ]]);
                    }]
                    [?(public) pub] [#field.name]: [field.ty().gen(out)],
                }]
            }
        }
    }

    fn generate_def_serde(&self, out: &mut Vec<TokenTree>, public: bool, flags: FLAGS) {
        if self.kind == ItemKind::Tuple {
            splat! {
                out;
                #[[ derive [emit_serde_derive(out, flags)] ]]
                [?(public) pub] struct [#self.name] (
                    [for (field in self.fields) {
                        [?(public) pub] [field.ty().gen(out)],
                    }]
                );
            }
            return;
        }
        splat! {
            out;
            #[[ derive [emit_serde_derive(out, flags)] ]]
            [?(public) pub] struct [#self.name] [?(self.lifetimes !=0) <~a>] {
                [for (field in self.fields) {
                    [?(field.ty().lifetimes() !=0) #[[serde(borrow)]]]
                    [if let Some(rename) = field.rename {
                        splat!(out; #[[ serde(rename = [#Literal::string(rename)]) ]]);
                    }]
                    [?(public) pub] [#field.name]: [field.ty().gen(out)],
                }]
            }
        }
    }
}

impl<'a> Enum<'a> {
    pub fn generate_def(&self, out: &mut Vec<TokenTree>, lib: &Libary) {
        self.generate_def_flagged(out, lib, false, self.flags)
    }

    pub fn generate_def_public(&self, out: &mut Vec<TokenTree>, lib: &Libary) {
        self.generate_def_flagged(out, lib, true, self.flags)
    }

    pub fn generate_def_with_flags(
        &self,
        out: &mut Vec<TokenTree>,
        lib: &Libary,
        public: bool,
        flags: FLAGS,
    ) {
        self.generate_def_flagged(out, lib, public, flags)
    }

    fn generate_def_flagged(
        &self,
        out: &mut Vec<TokenTree>,
        lib: &Libary,
        public: bool,
        flags: FLAGS,
    ) {
        match lib {
            Libary::Jsony { .. } => self.generate_def_jsony(out, public, flags),
            Libary::Nanoserde => self.generate_def_nanoserde(out, public, flags),
            Libary::Musli => self.generate_def_musli(out, public, flags),
            Libary::Serde | Libary::Sonic | Libary::Miniserde | Libary::Midiserde => {
                self.generate_def_serde(out, public, flags)
            }
            Libary::Facet => self.generate_def_facet(out, public),
            Libary::Baseline => self.generate_def_baseline(out, public),
            Libary::Merde => {
                panic!("enum generate_def not supported for {:?}", lib.name())
            }
        }
    }

    fn emit_variant_fields_jsony(out: &mut Vec<TokenTree>, variant: &crate::schema::EnumVariant) {
        out.push(variant.name.clone().into());
        if variant.fields.is_empty() {
            splat!(out; ,);
        } else {
            let start = out.len();
            for field in variant.fields {
                if let Some(rename) = field.rename {
                    splat!(out; #[[ jsony(rename = [#Literal::string(rename)]) ]]);
                }
                splat!(out; [#field.name]: [field.ty().gen(out)],);
            }
            let inner = TokenStream::from_iter(out.drain(start..));
            out.push(TokenTree::Group(Group::new(Delimiter::Brace, inner)));
            splat!(out; ,);
        }
    }

    fn generate_def_jsony(&self, out: &mut Vec<TokenTree>, public: bool, flags: FLAGS) {
        splat! {
            out;
            #[[ derive(Jsony) ]]
            #[[ jsony [emit_jsony_flags(out, flags)] ]]
            [?(public) pub] enum [#self.name] {
                [for (variant in &self.variants) {
                    [Self::emit_variant_fields_jsony(out, variant)]
                }]
            }
        }
    }

    fn emit_variant_fields_serde(out: &mut Vec<TokenTree>, variant: &crate::schema::EnumVariant) {
        out.push(variant.name.clone().into());
        if variant.fields.is_empty() {
            splat!(out; ,);
        } else {
            let start = out.len();
            for field in variant.fields {
                if let Some(rename) = field.rename {
                    splat!(out; #[[ serde(rename = [#Literal::string(rename)]) ]]);
                }
                splat!(out; [#field.name]: [field.ty().gen(out)],);
            }
            let inner = TokenStream::from_iter(out.drain(start..));
            out.push(TokenTree::Group(Group::new(Delimiter::Brace, inner)));
            splat!(out; ,);
        }
    }

    fn generate_def_serde(&self, out: &mut Vec<TokenTree>, public: bool, flags: FLAGS) {
        splat! {
            out;
            #[[ derive [emit_serde_derive(out, flags)] ]]
            [?(public) pub] enum [#self.name] {
                [for (variant in &self.variants) {
                    [Self::emit_variant_fields_serde(out, variant)]
                }]
            }
        }
    }

    fn emit_variant_fields_nanoserde(
        out: &mut Vec<TokenTree>,
        variant: &crate::schema::EnumVariant,
    ) {
        out.push(variant.name.clone().into());
        if variant.fields.is_empty() {
            splat!(out; ,);
        } else {
            let start = out.len();
            for field in variant.fields {
                if let Some(rename) = field.rename {
                    splat!(out; #[[ nserde(rename = [#Literal::string(rename)]) ]]);
                }
                splat!(out; [#field.name]: [field.ty().gen(out)],);
            }
            let inner = TokenStream::from_iter(out.drain(start..));
            out.push(TokenTree::Group(Group::new(Delimiter::Brace, inner)));
            splat!(out; ,);
        }
    }

    fn generate_def_nanoserde(&self, out: &mut Vec<TokenTree>, public: bool, flags: FLAGS) {
        splat! {
            out;
            #[[ derive [emit_nanoserde_derive(out, flags)] ]]
            [?(public) pub] enum [#self.name] {
                [for (variant in &self.variants) {
                    [Self::emit_variant_fields_nanoserde(out, variant)]
                }]
            }
        }
    }

    fn emit_variant_fields_musli(out: &mut Vec<TokenTree>, variant: &crate::schema::EnumVariant) {
        out.push(variant.name.clone().into());
        if variant.fields.is_empty() {
            splat!(out; ,);
        } else {
            let start = out.len();
            for field in variant.fields {
                if let Some(rename) = field.rename {
                    splat!(out; #[[ musli(Text, name = [#Literal::string(rename)]) ]]);
                }
                splat!(out; [#field.name]: [field.ty().gen(out)],);
            }
            let inner = TokenStream::from_iter(out.drain(start..));
            out.push(TokenTree::Group(Group::new(Delimiter::Brace, inner)));
            splat!(out; ,);
        }
    }

    fn generate_def_musli(&self, out: &mut Vec<TokenTree>, public: bool, flags: FLAGS) {
        splat! {
            out;
            #[[ derive [emit_musli_derive(out, flags)] ]]
            [?(public) pub] enum [#self.name] {
                [for (variant in &self.variants) {
                    [Self::emit_variant_fields_musli(out, variant)]
                }]
            }
        }
    }

    fn emit_variant_fields_facet(out: &mut Vec<TokenTree>, variant: &crate::schema::EnumVariant) {
        out.push(variant.name.clone().into());
        if variant.fields.is_empty() {
            splat!(out; ,);
        } else {
            let start = out.len();
            for field in variant.fields {
                if let Some(rename) = field.rename {
                    splat!(out; #[[ facet(rename = [#Literal::string(rename)]) ]]);
                }
                splat!(out; [#field.name]: [field.ty().gen(out)],);
            }
            let inner = TokenStream::from_iter(out.drain(start..));
            out.push(TokenTree::Group(Group::new(Delimiter::Brace, inner)));
            splat!(out; ,);
        }
    }

    fn generate_def_facet(&self, out: &mut Vec<TokenTree>, public: bool) {
        splat! {
            out;
            #[[ derive(Facet) ]]
            #[[ repr(C) ]]
            [?(public) pub] enum [#self.name] {
                [for (variant in &self.variants) {
                    [Self::emit_variant_fields_facet(out, variant)]
                }]
            }
        }
    }

    fn emit_variant_fields_baseline(
        out: &mut Vec<TokenTree>,
        variant: &crate::schema::EnumVariant,
    ) {
        out.push(variant.name.clone().into());
        if variant.fields.is_empty() {
            splat!(out; ,);
        } else {
            let start = out.len();
            for field in variant.fields {
                splat!(out; [#field.name]: [field.ty().gen(out)],);
            }
            let inner = TokenStream::from_iter(out.drain(start..));
            out.push(TokenTree::Group(Group::new(Delimiter::Brace, inner)));
            splat!(out; ,);
        }
    }

    fn generate_def_baseline(&self, out: &mut Vec<TokenTree>, public: bool) {
        splat! {
            out;
            [?(public) pub] enum [#self.name] {
                [for (variant in &self.variants) {
                    [Self::emit_variant_fields_baseline(out, variant)]
                }]
            }
        }
    }
}
