use std::str::FromStr;
use proc_macro2::token_stream::TokenStream as TkStream;

 

const TRAIT_INFO_CODE1: &'static str = "
impltrait[
    impl CashFlow for MyType 
    {
       type Coin = Subtype::Peso;
       type Bank = Subtype::JPMorgan;
       const EXCHANGE_RATE:f64 = 59.3;
       
       fn down_payment(&self, target:Client) -> f64
       {
           targer.cash + self.payment
       }
    }
];
submember_id;
traitdef[
     CashFlow {
        FUNCS[
            fn down_payment(&self, target:Client) -> f64;
            fn loan_shark<Guarantee:Client>(&self, amount:f64) -> impl Guarantee;
            fn discount_benefit(&mut self,amount:f64, salesman:&mut ClieNt) -> Option<String>;
        ]
        TYPES[Coin;Bank;]
        CONSTANTS[EXCHANGE_RATE;]
    }    
]
";

const TRAIT_INFO_CODE2: &'static str = "
header[
    MyStruct<'b,B:bound, C> where C:'b + bound
];
submember_id;
traitpath[momo::CashFlow];
traitdef[
     CashFlow {
        FUNCS[
            fn down_payment(&self, target:Client) -> f64;
            fn loan_shark<Guarantee:Client>(&self, amount:f64) -> impl Guarantee;
            fn discount_benefit(&mut self,amount:f64, salesman:&mut ClieNt) -> Option<String>;
        ]
        TYPES[Coin;Bank;]
        CONSTANTS[EXCHANGE_RATE;]
    }    
]
";


const TRAIT_INFO_CODE3: &'static str = "
impltrait[
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
];
submember_id;
traitdef[
     CashFlow {
        FUNCS[
            fn down_payment(&self, target:Client) -> f64;
            fn loan_shark<Guarantee:Client>(&self, amount:f64) -> impl Guarantee;
            fn discount_benefit(&mut self,amount:f64, salesman:&mut Client) -> Option<String>;
            fn price_oracle<Guarantee:Client>(&self) -> Option<String>;
            fn percentile_calc(base:f64, rate:f64) -> Option<u128>;
            fn credit_score_calc<Loan:Shark>(price:f64, rate:f64) -> Option<u128>;
        ]
        TYPES[Coin;Bank;]
        CONSTANTS[EXCHANGE_RATE;]
    }    
]
";


const TRAIT_INFO_CODE4: &'static str = "
header[
    MyStruct<'b,B:bound, C> where C:'b + bound
];
submember_id;
traitpath[module::CashFlow];
traitdef[
     CashFlow<'b, B:bound> {
        FUNCS[
            fn down_payment(&self, target:Client) -> f64;
            fn loan_shark<Guarantee:Client>(&self, amount:f64) -> impl Guarantee;
            fn discount_benefit(&mut self,amount:f64, salesman:&mut ClieNt) -> Option<String>;
        ]
        TYPES[Coin;Bank;]
        CONSTANTS[EXCHANGE_RATE;]
    }    
]
";


const TRAIT_INFO_CODE5: &'static str = "
header[
    MyStruct<'b,B:bound, C> where C:'b + bound
];
submember_id;
traitpath[::ops::CashFlow];
traitdef[
     CashFlow {
        FUNCS[
            fn down_payment(&self, target:Client) -> f64;
            fn loan_shark<Guarantee:Client>(&self, amount:f64) -> impl Guarantee;
            fn discount_benefit(&mut self,amount:f64, salesman:&mut ClieNt) -> Option<String>;
            fn percentile_calc(base:f64, rate:f64) -> Option<u128>;
        ]
        TYPES[Coin;Bank;]
        CONSTANTS[EXCHANGE_RATE;]
    }    
]
";

fn main() {
    let code_tokens1 = TkStream::from_str(TRAIT_INFO_CODE1).unwrap();
    let receiver_obj = syn::parse2::<forwarding_gen::ForwardingTraitImplReceiver>(code_tokens1).unwrap();

    let generated_code = receiver_obj.generate_impl_methods();

    match generated_code 
    {
        Ok(codedtokens) => {
            println!("\nImplemented Methods : \n {}\n", codedtokens);
        },
        Err(err) => {
            println!("\n Error with Trait implementation : \n {}\n", err);
        }
    };


    // Full derive trait
    let code_tokens2 = TkStream::from_str(TRAIT_INFO_CODE2).unwrap();
    let receiver_obj2 = syn::parse2::<forwarding_gen::ForwardingDeriveMemberReceiver>(code_tokens2).unwrap();

    let generated_code2 = receiver_obj2.generate_impl_methods();

    match generated_code2
    {
        Ok(codedtokens) => {
            println!("\nImplemented Derive : \n {}\n", codedtokens);
        },
        Err(err) => {
            println!("\n Error with Trait implementation : \n {}\n", err);
        }
    };

    /*********************/
    let code_tokens3 = TkStream::from_str(TRAIT_INFO_CODE3).unwrap();
    let receiver_obj3 = syn::parse2::<forwarding_gen::ForwardingTraitImplReceiver>(code_tokens3).unwrap();

    let generated_code3 = receiver_obj3.generate_impl_methods();

    match generated_code3 
    {
        Ok(codedtokens) => {
            println!("\nImplemented Methods : \n {}\n", codedtokens);
        },
        Err(err) => {
            println!("\n Error with Trait implementation : \n {}\n", err);
        }
    };

    /**********************/
    let code_tokens4 = TkStream::from_str(TRAIT_INFO_CODE4).unwrap();
    let receiver_obj4 = syn::parse2::<forwarding_gen::ForwardingDeriveMemberReceiver>(code_tokens4).unwrap();

    let generated_code4 = receiver_obj4.generate_impl_methods();

    // This should generate a Trait error
    match generated_code4
    {
        Ok(codedtokens) => {
            println!("\nImplemented Derive : \n {}\n", codedtokens);
        },
        Err(err) => {
            println!("\n Error with Trait implementation : \n {}\n", err);
        }
    };

    /**********************/
    let code_tokens5 = TkStream::from_str(TRAIT_INFO_CODE5).unwrap();
    let receiver_obj5 = syn::parse2::<forwarding_gen::ForwardingDeriveMemberReceiver>(code_tokens5).unwrap();

    let generated_code5 = receiver_obj5.generate_impl_methods();

    // This should generate a Trait error
    match generated_code5
    {
        Ok(codedtokens) => {
            println!("\nImplemented Derive : \n {}\n", codedtokens);
        },
        Err(err) => {
            println!("\n Error with Trait implementation : \n {}\n", err);
        }
    };


    
}
