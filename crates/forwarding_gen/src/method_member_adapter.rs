/*
Hereditary
Autors: Francisco Leon <https://github.com/superoptimo>
License Apache-2.0
*/

use proc_macro2::token_stream::TokenStream as TkStream;
use quote::ToTokens;
use syn::{Ident, Signature, Expr};
use proc_macro2::Span;
use sha3::{Digest, Sha3_256};
use base32ct::{Base32Unpadded, Encoding};

#[derive(Debug, Clone)]
pub(crate) enum MethodAdaptError
{
    NotDispatchable(Span),
    MalformedArgument(Span),
    SyntaxError(syn::Error)
}

impl From<MethodAdaptError> for syn::Error
{
    fn from(value: MethodAdaptError) -> Self {
        match value {
            MethodAdaptError::NotDispatchable(s) => syn::Error::new(s, "Method not dispatchable. Object-Safe Traits require a valid receiver."),
            MethodAdaptError::MalformedArgument(s) => syn::Error::new(s, "Malformed argument in Signature, identifier required."),
            MethodAdaptError::SyntaxError(err) => err
        }
    }
}

impl From<syn::Error> for MethodAdaptError
{
    fn from(value: syn::Error) -> Self {
        MethodAdaptError::SyntaxError(value)
    }
}

impl std::fmt::Display for MethodAdaptError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {        
        write!(f,"{}", syn::Error::from(self.clone()))
    }
}

fn generate_macro_signature_hash(
    submember:&Ident, 
    method_sig:&Signature,
    base_hash:&str
) -> String
{
    let strbulk = format!("member:{},signature:{},{}", 
        submember.to_string(),
        method_sig.to_token_stream(),
        base_hash
    );

    let mut hasher = Sha3_256::new();
    hasher.update(strbulk.as_bytes());
    let finv = hasher.finalize();
    Base32Unpadded::encode_string(&finv[..])
}

pub(crate) struct TraitMethodImplMacros
{
    pub(crate) macro_decl:TkStream,
    pub(crate) macro_invoke:TkStream
}

impl TraitMethodImplMacros
{
    pub(crate) fn create(
        struct_member: &Ident, 
        method_sig: &Signature,
        base_hash:&str
    ) -> Result<Self, MethodAdaptError>
    {

        let mut params_iter = method_sig.inputs.iter();

        // obtain first parameter
        let first_param = params_iter.next().ok_or(
            MethodAdaptError::NotDispatchable(method_sig.ident.span())
        )?;

        // check if first parameter is a valid receiver
        let receiver_pat:TkStream = match first_param {
            syn::FnArg::Receiver(rcv) => {
                if rcv.reference.is_some() {
                    let muttk = rcv.mutability;
                    let stks = quote::quote!(&#muttk);
                    Ok(stks)
                }
                else
                {
                    Err(MethodAdaptError::NotDispatchable(method_sig.ident.span()))
                }                
            },
            _ => Err(MethodAdaptError::NotDispatchable(method_sig.ident.span()))
        }?;

        
        // extract the rest of parameters names
        let parameters_pair: Vec<(Expr, syn::FnArg)> = params_iter.map(
            |arg| -> Result<(Expr, syn::FnArg), MethodAdaptError> {
                match arg {
                    syn::FnArg::Receiver(_) => {
                        // Method needs to be dispatchable
                        Err(MethodAdaptError::NotDispatchable(method_sig.ident.span()))
                    },
                    syn::FnArg::Typed(typed) => {
                        let innerpat = typed.pat.as_ref();
                        match innerpat {
                            syn::Pat::Ident(idx) => {
                                let ret_expr= syn::parse2::<Expr>(idx.ident.to_token_stream());
                                ret_expr.map(|expr | (expr, arg.clone() ) ).or_else(|err| Err(err.into()))                            
                            },
                            _ => {
                                Err(MethodAdaptError::MalformedArgument(method_sig.ident.span()))
                            }
                        }
                    }
                }
            }
        ).collect::<Result< _ , _> >()?;

        let (parameters, signature_params) : (Vec<Expr>, Vec<syn::FnArg>) = parameters_pair.into_iter().unzip();

        let method_name = &method_sig.ident;
        
        // If it doesn't have a return type, put a semi colon
        let method_output = method_sig.output.clone();
        let semi_end:Option<syn::token::Semi>  = match &method_output {
            syn::ReturnType::Default => {Some(syn::token::Semi::default())},
            _ => { None }
        };

        let method_generics = &method_sig.generics;
        let method_where = &method_sig.generics.where_clause;


        // calculate macro name with signature hash
        let inner_macro_method_name_str = format!("macromethod_{}_{}",
            method_name.to_string(),
            generate_macro_signature_hash(struct_member, method_sig, base_hash)
        );

        let inner_macro_method_name = Ident::new(&inner_macro_method_name_str, method_name.span());

        // macro declaration should be inserted outside trait implementation
        let mdecl = quote::quote!(
            macro_rules! #inner_macro_method_name {
                ($self_token:ident) => {
                    
                    fn #method_name #method_generics (#receiver_pat $self_token #(,#signature_params)* ) #method_output
                    #method_where
                    {
                        $self_token.#struct_member.#method_name( #(#parameters),* ) #semi_end
                    }
                };
            }
        );

        // macro invokation should be inserted inside trait implementation
        let minvoke = quote::quote!(#inner_macro_method_name!(self););

        Ok(Self{macro_decl:mdecl, macro_invoke:minvoke})

    }

}
