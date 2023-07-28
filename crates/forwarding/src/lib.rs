/*
Hereditary
Autors: Francisco Leon <https://github.com/superoptimo>
License Apache-2.0
*/

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn forward_trait(attribs:TokenStream, item:TokenStream) -> TokenStream
{
    let parse_status = forwarding_gen::ForwardingTraitImplInput::parse_from_streams(
        attribs.into(), item.into()
    );

    parse_status.and_then(
        |impl_input| Ok(impl_input.generate_macro_code())
    ).unwrap_or_else(
        |err| err.into_compile_error()
    ).into()
}

#[proc_macro_derive(Forwarding, attributes(forward_derive, forwarding_mod_path))]
pub fn forwarding_derive(input:TokenStream) -> TokenStream
{
    let parse_status = syn::parse2::<forwarding_gen::ForwardingDeriveInput>(input.into());

    parse_status.and_then(
        |derive_input| Ok(derive_input.generate_macro_code())
    ).unwrap_or_else(
        |err| err.into_compile_error()
    ).into()
}

#[proc_macro]
#[allow(non_snake_case)]
pub fn ForwardingTraitImplProcess(content:TokenStream) -> TokenStream
{
    let parse_status = syn::parse2::<forwarding_gen::ForwardingTraitImplReceiver>(content.into());
    parse_status.and_then(
        |receiver| receiver.generate_impl_methods()
    ).unwrap_or_else(
        |err| err.into_compile_error()
    ).into()
}


#[proc_macro]
#[allow(non_snake_case)]
pub fn ForwardingDeriveMemberProcess(content:TokenStream) -> TokenStream
{
    let parse_status = syn::parse2::<forwarding_gen::ForwardingDeriveMemberReceiver>(content.into());
    parse_status.and_then(
        |receiver| receiver.generate_impl_methods()
    ).unwrap_or_else(
        |err| err.into_compile_error()
    ).into()
}


