/*
Hereditary
Autors: Francisco Leon <https://github.com/superoptimo>
License Apache-2.0
*/

use proc_macro2::token_stream::TokenStream as TkStream;
use quote::ToTokens;
use syn::{ItemImpl, Ident};
use trait_info_gen::TraitPathAttrib as TraitPathAttrib;
use crate::forwarding_derive_input::FORWARD_DERIVE_PATH_ATTRIB as FORWARD_TRAIT_IMPL_PATH_ATTRIB;
use crate::forwarding_derive_input::FORWARD_DERIVE_PATH_DEFAULT as FORWARD_TRAIT_IMPL_PATH_DEFAULT;
use sha3::{Digest, Sha3_256};
use base32ct::{Base32Unpadded, Encoding};

pub const FORWARD_TRAIT_IMPL_PROCESS_MACRO_NAME: &'static str  = "ForwardingTraitImplProcess";

struct ForwardingTraitAttribParams
{
    submember_id:Ident,
    process_macro_path: syn::Path
}

fn parse_parenthized_mod_path(input: syn::parse::ParseStream) -> syn::Result<syn::Path>
{    
    let _comma = input.parse::<syn::token::Comma>()?;
    let param_id = input.parse::<Ident>()?;
    if param_id.eq(FORWARD_TRAIT_IMPL_PATH_ATTRIB) {
        let param_content;
        let _cparen = syn::parenthesized!(param_content in input);        
        param_content.parse::<syn::Path>()
    }
    else
    {
        syn::Result::Err(syn::Error::new(input.span(), "Path parameter malformed."))
    }
}

/// Parse from attribute params inner content
impl syn::parse::Parse for ForwardingTraitAttribParams
{
    /// Read the submember identifier, and the process_macro_path if any.
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let submemberid:Ident = input.parse()?;        
        
        let mut macropath: syn::Path = parse_parenthized_mod_path(input).unwrap_or(
            // default macro path is "hereditary"
            syn::Path::from(syn::Ident::new(FORWARD_TRAIT_IMPL_PATH_DEFAULT, submemberid.span()))
        );

        // append the macro processing function name
        macropath.segments.push(syn::Ident::new(FORWARD_TRAIT_IMPL_PROCESS_MACRO_NAME, submemberid.span()).into());

        Ok(Self{submember_id:submemberid, process_macro_path:macropath})
    }
}

pub struct ForwardingTraitImplInput
{    
    pub submember_id:Ident,
    pub trait_path:TraitPathAttrib,
    pub process_macro_path: syn::Path,
    pub trait_impl:ItemImpl,
}

impl ForwardingTraitImplInput
{
    /// This method instantiates ForwardingTraitImplInput from the attribute macro
    pub fn parse_from_streams(attrib_input:TkStream, item_input:TkStream) -> syn::Result<Self>
    {
        let attribparams = syn::parse2::<ForwardingTraitAttribParams>(attrib_input)?;

        // parse Trait Implementation
        let traitimpl = syn::parse2::<ItemImpl>(item_input)?;

        let tpath:syn::Path = if let Some((_,pobj, _)) = &traitimpl.trait_ {
            Ok(pobj.clone())
        }
        else
        {
            syn::Result::Err(syn::Error::new(traitimpl.impl_token.span.clone(), "Required a Trait name."))            
        }?;

        // extract trait path
        let traitpath = TraitPathAttrib::from_path::<true>(tpath, traitimpl.impl_token.span.clone())?;

        Ok(Self{
            submember_id:attribparams.submember_id,
            trait_path: traitpath,
            process_macro_path:attribparams.process_macro_path,
            trait_impl:traitimpl
        })
    }

    fn generate_macro_hash(&self) -> String
    {
        let strbulk = format!("member:{},traithpath:{},macropath{},impl{}", 
            self.submember_id.to_string(),
            self.trait_path.gen_path().to_token_stream(),
            self.process_macro_path.to_token_stream(),
            self.trait_impl.to_token_stream()
        );

        let mut hasher = Sha3_256::new();
        hasher.update(strbulk.as_bytes());
        let finv = hasher.finalize();
        Base32Unpadded::encode_string(&finv[..])
    }

    pub fn generate_macro_code(&self) -> TkStream
    {
        let bridge_macro_name_str = format!("ForwardingTraitImpl_{}", self.generate_macro_hash());
        let bridge_macro_id = syn::Ident::new(&bridge_macro_name_str.as_str(), self.submember_id.span());

        let receiver_macro = &self.process_macro_path;
        let member_id = &self.submember_id;
        let trait_impl_block = &self.trait_impl;
        let trait_info_macro = self.trait_path.gen_info_macro_path();

        // Bridge macro and the invocation from trait info
        quote::quote!(
            macro_rules! #bridge_macro_id
            {
                ([$member_id:ident],[$($traitinfo:tt)*]) => {
                    #receiver_macro!(impltrait[#trait_impl_block];$member_id;traitdef[$($traitinfo)*]);
                };
            }

            #trait_info_macro!(#bridge_macro_id,[#member_id]);
        )
    }
}