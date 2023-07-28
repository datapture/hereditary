/*
Hereditary
Autors: Francisco Leon <https://github.com/superoptimo>
License Apache-2.0
*/

use syn::DeriveInput;
use proc_macro2::token_stream::TokenStream as TkStream;
use trait_info_gen::TraitPathAttrib as TraitPathAttrib;
use syn::Ident;
use quote::ToTokens;
use sha3::{Digest, Sha3_256};
use base32ct::{Base32Unpadded, Encoding};

pub struct ForwardingDeriveMemberTask
{
    pub submember_id:Ident,
    pub trait_path:TraitPathAttrib
}

pub struct ForwardingDeriveInput
{
    pub type_id:Ident,
    pub generics_info:syn::Generics,
    pub member_tasks:Vec<ForwardingDeriveMemberTask>,
    pub process_macro_path: syn::Path
}

pub const FORWARD_DERIVE_PATH_ATTRIB: &'static str  = "forwarding_mod_path";
pub const FORWARD_DERIVE_PATH_DEFAULT: &'static str  = "hereditary";
pub const FORWARD_DERIVE_PROCESS_MACRO_NAME: &'static str  = "ForwardingDeriveMemberProcess";
pub const FORWARD_DERIVE_MEMBER_ATTRIB: &'static str  = "forward_derive";

/// Code generation
impl ForwardingDeriveInput
{
    fn generate_header_tokens(&self) -> TkStream
    {
        let typeid = &self.type_id;
        let genericsinfo = &self.generics_info;
        let whereop = &genericsinfo.where_clause;
        quote::quote!(#typeid #genericsinfo #whereop)
    }
    

    fn generate_macro_hash(&self) -> String
    {
        let header_tks = self.generate_header_tokens();
        let strbulkhead = format!("FORWARD-DERIVE=>[typeid_header:[{}],macropath:{}],tasks({})",
            header_tks,
            self.process_macro_path.to_token_stream(),
            self.member_tasks.len()
        );

        let mut hasher = Sha3_256::new();
        hasher.update(strbulkhead.as_bytes());

        // hash forwarding task collection
        for task in &self.member_tasks
        {
            let strtask = format!("member:{},trait{}", task.submember_id.to_string(), task.trait_path);
            hasher.update(strtask.as_bytes());
        }

        let finv = hasher.finalize();
        Base32Unpadded::encode_string(&finv[..])
    }

    /// Generates the macro bridge for receiving syntax information
    /// from the traits selected to be derived on members of the type struct.
    pub fn generate_macro_code(&self) -> TkStream
    {
        // 1) First generate the bridge macro name.
        let forward_input_hash = self.generate_macro_hash();
        let bridge_macro_name_str = format!("ForwardingDeriveBridge_{}", forward_input_hash);
        let bridge_macro_name_id = syn::Ident::new(&bridge_macro_name_str.as_str(), self.type_id.span());
        
        // 2) Generate invocations for trait tasks
        let invocations:Vec<TkStream> = self.member_tasks.iter().map(
            |derive_task| -> TkStream 
            {
                let trait_info_macro = derive_task.trait_path.gen_info_macro_path();
                let trait_path_full = derive_task.trait_path.gen_path();
                let member_id = &derive_task.submember_id;
                quote::quote!(#trait_info_macro!(#bridge_macro_name_id,[#member_id{#trait_path_full}]);)
            }
        ).collect();

        // 3) Generate the header expression of the type declaration
        let typeheaderblock = self.generate_header_tokens();

        // 4) Receiver macro name
        let receiver_macro = &self.process_macro_path;

        // 5) Compose macro with bridge and invocations        
        quote::quote!(
            macro_rules! #bridge_macro_name_id
            {
                ([$member_id:ident{$trait_path_full:path}],[$($traitinfo:tt)*]) => {
                    #receiver_macro!(header[#typeheaderblock];$member_id;traitpath[$trait_path_full];traitdef[$($traitinfo)*]);
                };
            }

            #(#invocations)*
        )
    }
}

/// Parsing
impl syn::parse::Parse for ForwardingDeriveInput
{
    /// Parses a derive input from a type struct
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {

        let structinput:DeriveInput = input.parse()?;
        // compose the path for processing the macro
        let mut macropath:syn::Path = structinput.attrs.iter().find_map(
            |ele|->Option<syn::Path> {
                if ele.path().is_ident(FORWARD_DERIVE_PATH_ATTRIB) {
                    ele.parse_args().ok()            
                }
                else{None}                
            }
        ).unwrap_or(
            // default macro path is "hereditary"
            syn::Path::from(syn::Ident::new(FORWARD_DERIVE_PATH_DEFAULT, structinput.ident.span()))
        );

        // append the macro processing function name
        macropath.segments.push(syn::Ident::new(FORWARD_DERIVE_PROCESS_MACRO_NAME, structinput.ident.span()).into());

        ///////////////
        // check members
        let mut tasks_list: Vec<ForwardingDeriveMemberTask> = Vec::new();
        match &structinput.data 
        {
            syn::Data::Struct(strobj) => {
                strobj.fields.iter().try_for_each(
                    |f| -> syn::Result<()> 
                    {
                        if let Some(field_id) =  &f.ident
                        {
                            // Attribute parameter should have trait paths
                            f.attrs.iter().try_for_each(
                                |attrib| ->syn::Result<()> {
                                    
                                    // check attribute if indicates a forward_derive
                                    if attrib.path().is_ident(FORWARD_DERIVE_MEMBER_ATTRIB) {

                                        attrib.parse_nested_meta(|meta| -> syn::Result<()> {
                                            let parsed_path = TraitPathAttrib::from_path::<false>(meta.path.clone(), field_id.span())?;
                                            tasks_list.push(ForwardingDeriveMemberTask{submember_id:field_id.clone(), trait_path:parsed_path});
                                            Ok(())
                                        }) // attrib.parse_nested_meta(|meta| -> syn::Result<()> {
                                    }else { Ok(()) }                                
                                } // |attrib| ->syn::Result<()> 
                            ) // f.attrs.iter().try_for_each
                        } // if let Some(field_id) =  &f.ident
                        else {
                            syn::Result::Err(syn::Error::new(structinput.ident.span(), "A named field is required."))                        
                        }
                    } // |f| -> syn::Result<()> 
                ) // strobj.fields.iter().try_for_each
            }, // syn::Data::Struct(strobj)
            _ => {syn::Result::Err(syn::Error::new(structinput.ident.span(), "An Struct Type is required."))}
        }?;
        // create header parametrs
        Ok(Self{
            type_id:structinput.ident,
            generics_info: structinput.generics,
            member_tasks: tasks_list,
            process_macro_path: macropath
        })
    }
}
