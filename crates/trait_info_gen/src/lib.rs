/*
Hereditary
Autors: Francisco Leon <https://github.com/superoptimo>
License Apache-2.0
*/

use proc_macro2::token_stream::TokenStream as TkStream;
use syn::ItemTrait;

mod simple_trait_info;
mod trait_path_attrib;

/// exports
pub use simple_trait_info::SimpleTraitInfo;
/// Utility for handling trait paths
pub use trait_path_attrib::TraitPathAttrib;

/// Internal  Macro processing
fn process_trait(trait_decl: ItemTrait) -> TkStream
{
    let traitvis = trait_decl.vis.clone();

    let macrovis = if let syn::Visibility::Public(_) = traitvis {
        quote::quote!{#[macro_export]}
    }
    else {quote::quote!{}};
    
    let trait_macro_name = syn::Ident::new(format!("TraitInfo_{}", trait_decl.ident).as_str(), trait_decl.ident.span());

    let simp_trait_info = SimpleTraitInfo::create(&trait_decl);

    // Macro name with base32 encoding hash suffix
    let base32hash_tinfo = simp_trait_info.hash_base32();
    let trait_macro_name_real = syn::Ident::new(format!("TraitInfo{}_{}", base32hash_tinfo,  trait_decl.ident).as_str(), trait_decl.ident.span());

    quote::quote!(
        #trait_decl

        #macrovis
        macro_rules! #trait_macro_name_real {
            ($target_bridge_macro:ident, [ $($bridgecontent:tt)*]) => {
                $target_bridge_macro!([$($bridgecontent)*],[ #simp_trait_info ]);
            };
        }

        #traitvis use #trait_macro_name_real as #trait_macro_name;
    )

}

/// Entry point for macro trait info generation
pub fn trait_info_codegen(input:TkStream) -> TkStream
{
    let parse_status = syn::parse2::<ItemTrait>(input);
    match parse_status {
        syn::Result::Ok(tinfo) => {
            process_trait(tinfo)
        },
        syn::Result::Err(errobj) =>{
            errobj.into_compile_error()
        }
    }
}

