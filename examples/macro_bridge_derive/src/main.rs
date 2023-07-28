use std::str::FromStr;
use proc_macro2::token_stream::TokenStream as TkStream;

const TRAIT_INFO_CODE1: &'static str = "
    struct MyType<'b, B, F>
        where B: bound + 'b,    
        F: Fn(u32,f64)->String
    {
        #[forward_derive(CashFlow, Netcode)]
        member1:Subclass,
        member2:Arc<Subclass2>,
        #[forward_derive(fmt::Display, typenum::MathOp)]
        member3:Box<Subclass3>,
    }
";

const TRAIT_INFO_CODE2: &'static str = "
    #[forwarding_mod_path(forwarding::inner)]
    struct MyType2
    {
        #[forward_derive(CashFlow, core::Netcode)]
        member1:Subclass,
        member2:Arc<Subclass2>,
        #[forward_derive(typenum::MathOp)]
        member3:Box<Subclass3>,
    }
";


fn main() {    
    let code_tokens1 = TkStream::from_str(TRAIT_INFO_CODE1).unwrap();

    let parse_status = syn::parse2::<forwarding_gen::ForwardingDeriveInput>(code_tokens1);

    let generated_code = parse_status.and_then(
        |impl_input| Ok(impl_input.generate_macro_code())
    ).unwrap_or_else(
        |err| err.into_compile_error()
    );

    println!("Generated Macros : \n {}\n", generated_code);

    // alternative with forwarding module path

    let code_tokens2 = TkStream::from_str(TRAIT_INFO_CODE2).unwrap();

    let parse_status2 = syn::parse2::<forwarding_gen::ForwardingDeriveInput>(code_tokens2);

    let generated_code2 = parse_status2.and_then(
        |impl_input| Ok(impl_input.generate_macro_code())
    ).unwrap_or_else(
        |err| err.into_compile_error()
    );

    println!("Generated Macros2 : \n {}\n", generated_code2);
}
