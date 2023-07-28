/*
Hereditary
Autors: Francisco Leon <https://github.com/superoptimo>
License Apache-2.0
*/

use proc_macro2::token_stream::TokenStream as TkStream;
use syn::Ident;
use quote::ToTokens;
use proc_macro2::Span as Span;


/// Utility class for parsing trait path referencing 
pub struct TraitPathAttrib
{
    pub base_path:syn::Path,
    pub trait_name:Ident,
    pub trait_path_args: syn::PathArguments
}

impl TraitPathAttrib
{
    /// generates the original path
    pub fn gen_path(&self) -> syn::Path
    {
        let spath = &self.base_path;
        let trait_name = &self.trait_name;
        if spath.leading_colon.is_some() || spath.segments.len() > 0
        {
            let mut newpath = spath.clone();
            newpath.segments.push(trait_name.clone().into());
            newpath
        }
        else
        {
            syn::Path::from(trait_name.clone())
        }
    }

    /// Generates a path with the last identifier is named
    /// `TraitInfo_<TraitName>`
    pub fn gen_info_macro_path(&self) -> TkStream
    {
        // obtain base path
        let mut spath = self.base_path.clone();

        // generate trait macro name
        let trait_name = &self.trait_name;        
        let macro_name = syn::Ident::new(format!("TraitInfo_{}", trait_name).as_str(), trait_name.span());

        // re-append the trait macro identifier
        spath.segments.push(macro_name.into());

        return spath.into_token_stream()
    }

    pub fn from_path<const ALLOW_GENERICS:bool>(mut path:syn::Path, span:Span) -> syn::Result<Self>
    {   
        // error object message
        let errobj = syn::Error::new(span, 
            if ALLOW_GENERICS {"Malformed Path."}
            else {"Requires a concrete path without arguments."});
        // take the last identifier
        let traitname_op = path.segments.pop();
        
        let (traitname, pargs) = if let Some(pair) = traitname_op {
            if ALLOW_GENERICS == false {
                if pair.value().arguments.is_empty() == true
                {
                    // requires concrete traits
                    Some((pair.value().ident.clone(), syn::PathArguments::None))
                }
                else{None}
            }
            else
            {
                // allow generics
                Some((pair.value().ident.clone(), pair.value().arguments.clone()))
            }            
        }
        else{None}.ok_or(errobj)?;
        
        Ok(Self{base_path:path, trait_name:traitname, trait_path_args:pargs})
    }
}

/// Parse TraitPathAttrib as syn::Path instance
impl syn::parse::Parse for TraitPathAttrib
{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let basepath = input.parse::<syn::Path>()?;        
        TraitPathAttrib::from_path::<true>(basepath, input.span())
    }
}

impl std::fmt::Display for TraitPathAttrib
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{{base:{}", self.base_path.to_token_stream())?;
        write!(f,",trait_name:{}", self.trait_name.to_string())?;
        write!(f,",args:{} }}", self.trait_path_args.to_token_stream())
    }
}