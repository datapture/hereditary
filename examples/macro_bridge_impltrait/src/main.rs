use std::str::FromStr;
use proc_macro2::token_stream::TokenStream as TkStream;

const TRAIT_INFO_CODE1: &'static str = "
    impl<'b, B, F, const TETA:u32> CashFlow<F,TETA> for MyType<'b, B> 
        where B: bound + 'b,    
        F: Fn(u32,f64)->String
    {
       type Coin = B;
       type Bank = F;
       const EXCHANGE_RATE:u32 = TETA*2;
       
       fn down_payment(&self, target:Client) -> f64
       {
           targer.cash + self.payment*Self::EXCHANGE_RATE
       }

       fn creditrate_calc<Loan:Shark>(price:f64, rate:f64) -> Option<u128>
       {
            Some(Self::EXCHANGE_RATE*rate - price)
       }
    }
";


const TRAIT_INFO_CODE2: &'static str = "
    impl package::CashFlow for MyType        
    {       
       fn down_payment(&self, target:Client) -> f64
       {
           targer.cash + self.payment*Self::EXCHANGE_RATE
       }

       fn creditrate_calc<Loan:Shark>(price:f64, rate:f64) -> Option<u128>
       {
            Some(Self::EXCHANGE_RATE*rate - price)
       }
    }
";


fn main() {
    let attrib_tokens1 = TkStream::from_str("member1").unwrap();
    let code_tokens1 = TkStream::from_str(TRAIT_INFO_CODE1).unwrap();

    let parse_status = forwarding_gen::ForwardingTraitImplInput::parse_from_streams(
        attrib_tokens1, code_tokens1
    );

    let generated_code = parse_status.and_then(
        |impl_input| Ok(impl_input.generate_macro_code())
    ).unwrap_or_else(
        |err| err.into_compile_error()
    );

    println!("Generated Macros : \n {}\n", generated_code);

    // Alternative
    let attrib_tokens2 = TkStream::from_str("member_x, forwarding_mod_path(::heredetary::forwarding)").unwrap();
    let code_tokens2 = TkStream::from_str(TRAIT_INFO_CODE2).unwrap();

    let parse_status2 = forwarding_gen::ForwardingTraitImplInput::parse_from_streams(
        attrib_tokens2, code_tokens2
    );

    let generated_code2 = parse_status2.and_then(
        |impl_input| Ok(impl_input.generate_macro_code())
    ).unwrap_or_else(
        |err| err.into_compile_error()
    );

    println!("Generated Macros2 : \n {}\n", generated_code2);
}
