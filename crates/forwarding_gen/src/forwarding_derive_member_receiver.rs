/*
Hereditary
Autors: Francisco Leon <https://github.com/superoptimo>
License Apache-2.0
*/

use proc_macro2::token_stream::TokenStream as TkStream;
use syn::{Ident,Path};

use crate::method_member_adapter::TraitMethodImplMacros;

struct GenericParamsIntancing
{
    lifetimes:Vec<syn::Lifetime>,
    params:Vec<Ident>,
    constants:Vec<Ident>
}

impl GenericParamsIntancing
{
    fn create(generics_info:&syn::Generics) -> Self {
        let mut lifetimes: Vec<syn::Lifetime> = Vec::new();
        let mut params: Vec<Ident> = Vec::new();
        let mut constants: Vec<Ident> = Vec::new();
        generics_info.params.iter().for_each(|param| {
            match param {
                syn::GenericParam::Lifetime(lif) => {
                    lifetimes.push(lif.lifetime.clone());
                },
                syn::GenericParam::Type(typ) => {
                    params.push(typ.ident.clone());
                },
                syn::GenericParam::Const(cns) => {
                    constants.push(cns.ident.clone());
                },
            };
        });

        Self{lifetimes, params, constants}
    }

    #[allow(dead_code)]
    fn has_generic_arguments(&self) -> bool {
        if self.lifetimes.len() + self.params.len() + self.constants.len() > 0 {true} else {false}
    }

    fn generate_tokens(&self) -> TkStream 
    {
        let lifetimes = &self.lifetimes;
        let params = &self.params;
        let constants = &self.constants;
        if lifetimes.len() + params.len() + constants.len() == 0 {
            quote::quote!()
        }
        else {
            let lifetime_comma:Option<syn::token::Comma> = if lifetimes.len() > 0 && ( (params.len() + constants.len()) > 0) {
                Some(syn::token::Comma::default())
            } else {None};

            let params_comma:Option<syn::token::Comma> = if params.len() > 0 && constants.len() > 0 {
                Some(syn::token::Comma::default())
            } else {None};

            quote::quote!(<#(#lifetimes),* #lifetime_comma #(#params),*  #params_comma #(#constants),*>)
        }
    }
}

/// Syntax structure for processing method forwarding of traits implemented in submembers, where it takes a trait info from macro expression.
/// 
/// This utility class generates the entire set of methods for a determined trait, given the previously recorded trait information and
/// the subelement in the type struct that implements that trait.
pub struct ForwardingDeriveMemberReceiver
{
    pub type_id:Ident,
    pub generics_info:syn::Generics,
    pub submember_id:Ident,
    pub trait_path:Path,
    pub trait_info_obj:trait_info_gen::SimpleTraitInfo
}

impl ForwardingDeriveMemberReceiver
{
    pub fn generate_impl_methods(&self) -> syn::Result<TkStream>
    {
        let typeid = &self.type_id;
        if self.trait_info_obj.generics.params.len() > 0 {
            return syn::Result::Err(syn::Error::new(typeid.span(), "Cannot implement a trait with generic arguments."));
        }

        let trait_path = &self.trait_path;

        let impl_trait_header_decl: TkStream = if self.generics_info.params.len() > 0 {
    
            let generic_params_decl = &self.generics_info.params;
            // declare with generic arguments
            let generic_params_inst = GenericParamsIntancing::create(&self.generics_info).generate_tokens();
    
            let wherecause_tks = &self.generics_info.where_clause;
                            
            quote::quote!(
                impl< #generic_params_decl > #trait_path for #typeid #generic_params_inst #wherecause_tks
            )
        }
        else {
            quote::quote!(
                impl #trait_path for #typeid
            )
        };

        let base_hash = impl_trait_header_decl.to_string();
    
        // generate methods
        let impl_method_pairs:Vec<TraitMethodImplMacros> = self.trait_info_obj.functions.iter().map(
            |fsig| TraitMethodImplMacros::create(
                &self.submember_id, fsig, &base_hash
            ).or_else(|err| Err(err.into()) )
        ).collect::<syn::Result<_> >()?;

        // separate streams
        let (impl_method_decls, impl_method_invoks):
        (Vec<TkStream>,Vec<TkStream>) = impl_method_pairs.into_iter().map(
            |pair| (pair.macro_decl, pair.macro_invoke)
        ).unzip();
    
        let out_tokens = quote::quote!(
            #(#impl_method_decls)*

            #impl_trait_header_decl
            {
                #(#impl_method_invoks)*
            }
        );
        Ok(out_tokens)        
    }
}


impl syn::parse::Parse for ForwardingDeriveMemberReceiver
{
    /// Receives a macro syntax that contains the type struct declaration, the submember identifier and the trait definition.
    /// Parameters are separated by semi token `;`, where:
    /// * The first parameter is the declaration type header with generics (delimited by `header[]` block).
    /// * The second is the identifier of the submember that implements the trait methods.
    /// * The third is a trait information obtained by reflection macro before (delimited by `traitdef[]` block).
    /// 
    /// 
    /// ```
    /// ForwardingDeriveMemberProcess!(
    /// header[
    ///     MyStruct<'b,B:bound, C> where C:'b + bound
    /// ];
    /// submember_id;
    /// traitpath[CashFlow];
    /// traitdef[
    ///      unsafe CashFlow<'ar, Client:Sized> {
    ///         FUNCS[
    ///             fn down_payment(&self, target:Client) -> f64;
    ///             fn loan_shark<Guarantee:Client>(&self, amount:f64) -> impl Guarantee;
    ///             fn discount_benefit(amount:f64, salesman:&mut Cliet) -> Option<String>;
    ///         ]
    ///         TYPES[Coin;Bank;]
    ///         CONSTANTS[EXCHANGE_RATE;]
    ///     }    
    /// ]);
    /// ```
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        mod kw_inner
        {
            syn::custom_keyword!(header);
            syn::custom_keyword!(traitpath);
            syn::custom_keyword!(traitdef);
        }
        
        // header[]
        let _headtoken = input.parse::<kw_inner::header>()?;
        let typeheader_content;
        let _trait_brakets = syn::bracketed!(typeheader_content in input);

        let _sep0 = input.parse::<syn::token::Semi>()?; //;

        // sub member identifier
        let submember_id:Ident = input.parse()?;
        let _sep1 = input.parse::<syn::token::Semi>()?; //;

        // traitpath[]
        let _traitpath_token = input.parse::<kw_inner::traitpath>()?;
        let trait_path_content;
        let _trait_path_brakets = syn::bracketed!(trait_path_content in input);
        let _sep2 = input.parse::<syn::token::Semi>()?; //;
        
        // traitdef[]
        let _traitdeftoken = input.parse::<kw_inner::traitdef>()?;
        let trait_def_content;
        let _trait_def_brakets = syn::bracketed!(trait_def_content in input);

        // process the type struct header
        let typeid:Ident = typeheader_content.parse()?;
        let generics0:syn::Generics = typeheader_content.parse()?;
        let where0:Option<syn::WhereClause> = typeheader_content.parse()?;

        let genericsfinal = syn::Generics{where_clause:where0, ..generics0};

        // process the trait path
        let trait_path:Path = trait_path_content.parse()?;

        // process the trait info
        let trait_info_obj = trait_def_content.parse::<trait_info_gen::SimpleTraitInfo>()?;

        Ok(Self{type_id:typeid, generics_info:genericsfinal, submember_id, trait_path, trait_info_obj})
    }
}