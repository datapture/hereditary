/*
Hereditary
Autors: Francisco Leon <https://github.com/superoptimo>
License Apache-2.0
*/

use proc_macro::TokenStream;

/// 
/// Trait declarations only have to aggregate this attribute for generating 
/// trait information, and this procedure generates a declarative macro information
/// that pass trait information fields to a receiver bridge macro.
/// ```
/// #[trait_info]
/// trait MyTrait
/// {
///     type MyTyp = AnotherClass::Typ;
///     fn method1(&self, num:u32) -> String;
///     fn method1(&mut self, num:u32, arr:&[u8]) -> String;
/// }
/// ```
/// The resulting code processed by **Rust Compiler** expands a
/// delcarative macro that injects a trait information syntax to 
/// a requester macro by demand (The `Bridge Macro`):
/// 
/// ```
/// // Original trait
/// trait MyTrait
/// {
///     type MyTyp = AnotherClass::Typ;
///     fn method1(&self, num:u32) -> String;
///     fn method1(&mut self, num:u32, arr:&[u8]) -> String;
/// }
/// 
/// // Generated macro is named with the prefix TraitInfo_
/// // prepended before the name of the trait (MyTrait)
/// #[macro_export]
/// macrorules! TraitInfo_MyTrait
/// {
///     ($target_bridge_macro:ident, [ $($bridgecontent:tt)*]) => {
///         $target_bridge_macro!(
///             [
///                 $($bridgecontent)*
///             ],
///             [
///                 //** Here comes the trait information block
///                 MyTrait
///                 {
///                     FUNCS[
///                         fn method1(&self, num:u32) -> String;
///                         fn method1(&mut self, num:u32, arr:&[u8]) -> String;
///                     ]
///                     TYPES[
///                         MyTyp;
///                     ]
///                     CONSTANTS[]
///                 }
///                 //** End Block Trait info
///             ]
///         );
///     };
/// }
/// 
/// ```
/// The `Bridge Macro` would be a procedural macro that interprets the trait information
/// with the help of the utility type `trait_info_gen::SimpleTraitInfo`
/// 
#[proc_macro_attribute]
pub fn trait_info(_attrib:TokenStream, item:TokenStream) ->TokenStream
{
    // Attribute doesn't carry any information, 
    // only needs the trait declaration.

    trait_info_gen::trait_info_codegen(item.into()).into()
}

