use std::str::FromStr;
use proc_macro2::token_stream::TokenStream as TkStream;

const TRAIT_CODE1: &'static str = "
    trait CashFlow<Client> {
        type Coin = f64;

        fn down_payment(&self, target:Client) -> f64;

        fn loan_shark<Guarantee:Client>(&self, amount:f64) -> impl Guarantee;
    }
";

const TRAIT_INFO_CODE1: &'static str = "
    unsafe CashFlow<'ar, Client:Sized> {
        FUNCS[
            fn down_payment(&self, target:Client) -> f64;
            fn loan_shark<Guarantee:Client>(&self, amount:f64) -> impl Guarantee;
            fn discount_benefit(amount:f64, salesman:&mut Cliet) -> Option<String>;
        ]
        TYPES[Coin;Bank;]
        CONSTANTS[EXCHANGE_RATE;]
    }
";

fn main() {
    let code_tokens0 = TkStream::from_str(TRAIT_CODE1).unwrap();
    let trait_info_result = trait_info_gen::trait_info_codegen(code_tokens0);
    println!("\nGENERATED TOKENS : {}\n", trait_info_result);

    let code_tokens1 = TkStream::from_str(TRAIT_INFO_CODE1).unwrap();
    let trait_info_struct:trait_info_gen::SimpleTraitInfo = syn::parse2(code_tokens1).unwrap();

    println!("\nPARSED INFO : {}\n", trait_info_struct);
}
