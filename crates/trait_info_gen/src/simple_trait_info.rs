/*
Hereditary
Autors: Francisco Leon <https://github.com/superoptimo>
License Apache-2.0
*/

use proc_macro2::token_stream::TokenStream as TkStream;
use quote::ToTokens;
use syn::{ItemTrait, TraitItem, Generics, Ident, Signature};
use sha3::{Digest, Sha3_256};
use base32ct::{Base32Unpadded, Encoding};
pub struct SimpleTraitInfo
{
    pub unsafety: bool,
    pub ident: Ident,
    pub generics: Generics,
    pub functions: Vec<Signature>,
    pub typedefs: Vec<Ident>,
    pub constants: Vec<Ident>
}

impl SimpleTraitInfo
{
    pub fn create(traitinfo: &ItemTrait) -> Self
    {
        let mut tr_funcs:Vec<Signature> = Vec::new();
        let mut tr_types:Vec<Ident> = Vec::new();
        let mut tr_consts:Vec<Ident> = Vec::new();

        traitinfo.items.iter().for_each(|tr_item|{
            match tr_item {
                TraitItem::Fn(trfn) => {
                    tr_funcs.push(trfn.sig.clone());
                },
                TraitItem::Type(trty) => {
                    tr_types.push(trty.ident.clone());
                },
                TraitItem::Const(trconst) => {
                    tr_consts.push(trconst.ident.clone());
                },
                _ => {}
            };
        });
        
        SimpleTraitInfo{
            unsafety : traitinfo.unsafety.is_some(),
            ident: traitinfo.ident.clone(),
            generics: traitinfo.generics.clone(),
            functions: tr_funcs,
            typedefs:tr_types,
            constants:tr_consts
        }
    }

    pub fn hash_base32(&self) -> String
    {
        let sval = format!("{}", self);
        let mut hasher = Sha3_256::new();
        hasher.update(sval.as_bytes());
        let finv = hasher.finalize();
        Base32Unpadded::encode_string(&finv[..])
    }

    /// Select the functions that aren't mentioned in the `excluding_set`
    pub fn filter_functions(&self, excluding_set:&[Ident]) -> Vec<usize>
    {
        self.functions.iter().enumerate().filter_map(|(i, fsig)| -> Option<usize> {
            if excluding_set.contains(&fsig.ident) {None} else {Some(i)}
        }).collect()
    }
}

impl quote::ToTokens for SimpleTraitInfo
{
    fn to_tokens(&self, tokens: &mut TkStream) {
        let unsafekey = if self.unsafety == false { quote::quote!{} } else { quote::quote!{unsafe} };
        let trait_name = &self.ident;
        let trait_generics = &self.generics;
        let trait_funcs = &self.functions;
        let trait_types = &self.typedefs;
        let trait_constants = &self.constants;

        let trait_tokens = quote::quote!{
            #unsafekey #trait_name #trait_generics {
                FUNCS[
                    #(#trait_funcs;)*
                ]
                TYPES[
                    #(#trait_types;)*
                ]
                CONSTANTS[
                    #(#trait_constants;)*
                ]
            }
        };

        tokens.extend(trait_tokens);
    }
}

impl std::fmt::Display for SimpleTraitInfo
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let generics_tks = self.generics.to_token_stream();
        write!(f,"TraitInfo{{
    trait_name:{},
    unsafe:{},
    generics:[{}],",self.ident.to_string(), self.unsafety, generics_tks)?;

        writeln!(f,"\n    FUNCS[")?;

        for fi in &self.functions {
            let fn_tks = fi.to_token_stream();
            writeln!(f,"        {};", fn_tks)?;
        }

        writeln!(f,"    ]")?;
        writeln!(f,"    TYPES[")?;

        for ti in &self.typedefs {            
            writeln!(f,"        {};", ti.to_string())?;
        }

        writeln!(f,"    ]")?;
        writeln!(f,"    CONSTANTS[")?;

        for ci in &self.constants {            
            writeln!(f,"        {};", ci.to_string())?;
        }

        writeln!(f,"    ]")?;
        write!(f,"}}")
    }
}


mod trait_inner
{
    syn::custom_keyword!(FUNCS);
    syn::custom_keyword!(TYPES);
    syn::custom_keyword!(CONSTANTS);

    pub type FuncsList = syn::punctuated::Punctuated<syn::Signature, syn::Token![;]>;

    pub type IdentList = syn::punctuated::Punctuated<syn::Ident, syn::Token![;]>;
}


impl syn::parse::Parse for SimpleTraitInfo
{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let unsafe_key:Option<syn::token::Unsafe>  = input.parse()?;
        let trait_name:Ident = input.parse()?;
        let trait_generics:Generics = input.parse()?;
        let trait_content;
        let _openbrace0 = syn::braced!(trait_content in input);

        // read functions
        let _fn_kw = trait_content.parse::<trait_inner::FUNCS>()?;
        let funcs_content;
        let _openbrace1 = syn::bracketed!(funcs_content in trait_content);
        let funcs_signature = trait_inner::FuncsList::parse_terminated(&funcs_content)?;
        let funcs_signature:Vec<Signature> = funcs_signature.into_iter().collect();
        
        // read types
        let _ty_kw = trait_content.parse::<trait_inner::TYPES>()?;
        let types_content;
        let _openbrace2 = syn::bracketed!(types_content in trait_content);
        let types_list = trait_inner::IdentList::parse_terminated(&types_content)?;
        let types_list:Vec<Ident> = types_list.into_iter().collect();

        // read constants
        let _ty_kw = trait_content.parse::<trait_inner::CONSTANTS>()?;
        let constants_content;
        let _openbrace3 = syn::bracketed!(constants_content in trait_content);
        let constants_list = trait_inner::IdentList::parse_terminated(&constants_content)?;
        let constants_list:Vec<Ident> = constants_list.into_iter().collect();

        syn::Result::Ok(SimpleTraitInfo{
            unsafety : unsafe_key.is_some(),
            ident: trait_name,
            generics: trait_generics,
            functions: funcs_signature,
            typedefs:types_list,
            constants:constants_list
        })
    }
}