/*
Hereditary
Autors: Francisco Leon <https://github.com/superoptimo>
License Apache-2.0
*/

use proc_macro2::token_stream::TokenStream as TkStream;
use quote::ToTokens;
use syn::{ItemImpl, Ident, ImplItem};
use sha3::{Digest, Sha3_256};
use base32ct::{Base32Unpadded, Encoding};

use crate::method_member_adapter::{TraitMethodImplMacros, MethodAdaptError};

/// Registers already implemented methods from trait implementation expression
#[allow(dead_code)]
struct ImplementedItems
{
    methods:Vec<Ident>,
    types:Vec<Ident>,
    constants:Vec<Ident>,
    base_hash:String
}

impl ImplementedItems
{
    /// Accounts the identifiers from implemented methods in the trait implementation expression
    fn create(trait_impl:&ItemImpl) -> ImplementedItems
    {
        let mut methods:Vec<Ident> = Vec::new();
        let mut types:Vec<Ident> = Vec::new();
        let mut constants:Vec<Ident> = Vec::new();

        trait_impl.items.iter().for_each(|it| {
            match it {
                ImplItem::Fn(fnobj) => {
                    methods.push(fnobj.sig.ident.clone());
                },
                ImplItem::Type(tobj) => {
                    types.push(tobj.ident.clone());
                },
                ImplItem::Const(ctobj) => {
                    constants.push(ctobj.ident.clone());
                },
                _ =>{}
            };
        });

        // calculate hash
        let strbulk = format!("impl:{}", trait_impl.to_token_stream());

        let mut hasher = Sha3_256::new();
        hasher.update(strbulk.as_bytes());
        let finv = hasher.finalize();
        let strhash = Base32Unpadded::encode_string(&finv[..]);

        ImplementedItems{methods, types, constants, base_hash:strhash}
    }

    fn generate_trait_methods(
        &self,
        submember:&Ident,
        trait_info_obj: &trait_info_gen::SimpleTraitInfo
    ) -> syn::Result< Vec<TraitMethodImplMacros> >
    {
        trait_info_obj.functions.iter().filter(
            |&fsig| self.methods.contains(&fsig.ident) == false
        ).filter_map(
            |fsig| -> Option<syn::Result<TraitMethodImplMacros>> {
                // Avoid collecting non dispatchable
                match TraitMethodImplMacros::create(submember, fsig, &self.base_hash) {
                    Ok(stk) => { Some(Ok(stk)) },
                    Err(MethodAdaptError::NotDispatchable(_)) => {None},
                    Err(err) => { Some( Err(err.into()) ) }                    
                }
            }
        ).collect::<syn::Result<_> >()
    }
}

/// Syntax structure for receiving the implemented traits and the trait info from macro expression
pub struct ForwardingTraitImplReceiver
{
    pub trait_impl:ItemImpl,
    pub submember_id:Ident,
    pub trait_info_obj:trait_info_gen::SimpleTraitInfo
}

impl ForwardingTraitImplReceiver
{
    /// Builds a trait implementation with new additional method wrappers as items.
    pub fn generate_impl_methods(&self) -> syn::Result<TkStream>
    {
        let extended_method_pairs = ImplementedItems::create(
            &self.trait_impl
        ).generate_trait_methods(&self.submember_id, &self.trait_info_obj)?;

        if extended_method_pairs.len() == 0 {
            Ok(self.trait_impl.to_token_stream())
        }
        else {

            // separate extended methods
            let (extended_method_decls, extended_method_invoks):
            (Vec<TkStream>,Vec<TkStream>) = extended_method_pairs.into_iter().map(
                |pair| (pair.macro_decl, pair.macro_invoke)
            ).unzip();

            // a new instance of implementation
            let mut extended_trait_impl = self.trait_impl.clone();
            // aggregate the new methods    
            extended_trait_impl.items.extend(extended_method_invoks.into_iter().map(|fstr| -> ImplItem {
                ImplItem::Verbatim(fstr)
            }));

            let ret_tks = quote::quote!(
                #(#extended_method_decls)*

                #extended_trait_impl
            );

            Ok(ret_tks)
        }
    }
}

impl syn::parse::Parse for ForwardingTraitImplReceiver
{
    /// Process a macro syntax that contains the trait implementation, the submember identifier and the trait definition.
    /// Parameters are separated by semi token `;`, where:
    /// * The first parameter is implemented trait (delimited by `impltrait[]` block).
    /// * The second is the identifier of the submember that implements the trait methods.
    /// * The third is a trait information obtained by reflection macro before (delimited by `traitdef[]` block).
    /// 
    /// 
    /// ```
    /// ForwardingTraitImplProcess!(
    /// impltrait[
    ///     impl CashFlow for Type 
    ///     {
    ///         fn down_payment(&self, target:Client) -> f64
    ///         {
    ///             targer.cash + self.payment
    ///         }
    ///     }
    /// ];
    /// submember_id;
    /// traitdef[
    ///      CashFlow {
    ///         FUNCS[
    ///             fn down_payment(&self, target:Client) -> f64;
    ///             fn loan_shark<Guarantee:Client>(&self, amount:f64) -> impl Guarantee;
    ///             fn discount_benefit(&mut self,amount:f64, salesman:&mut Cliet) -> Option<String>;
    ///         ]
    ///         TYPES[Coin;Bank;]
    ///         CONSTANTS[EXCHANGE_RATE;]
    ///     }    
    /// ]);
    /// ```
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        mod kw_inner
        {
            syn::custom_keyword!(impltrait);
            syn::custom_keyword!(traitdef);            
        }
        
        // impltrait[]
        let _impltoken = input.parse::<kw_inner::impltrait>()?;
        let trait_impl_content;
        let _trait_brakets = syn::bracketed!(trait_impl_content in input);

        let _sep0 = input.parse::<syn::token::Semi>()?; //;

        // sub member identifier
        let submember_id:Ident = input.parse()?;
        let _sep1 = input.parse::<syn::token::Semi>()?; //;
        
        // traitdef[]
        let _traitdeftoken = input.parse::<kw_inner::traitdef>()?;
        let trait_def_content;
        let _trait_def_brakets = syn::bracketed!(trait_def_content in input);

        // process the trait implementation
        let trait_impl = trait_impl_content.parse::<syn::ItemImpl>()?;

        // process the trait info
        let trait_info_obj = trait_def_content.parse::<trait_info_gen::SimpleTraitInfo>()?;

        Ok(Self{trait_impl:trait_impl, submember_id, trait_info_obj})
        
    }
}
