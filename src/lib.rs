/*
Hereditary
Autors: Francisco Leon <https://github.com/superoptimo>
License Apache-2.0
*/

//! # Hereditary
//! Procedural macros for emulating OOP Inheritance in Rust, 
//! by extending the trait functionality in structs based on their composition
//! (as they contain instances of other types that implement the desired functionality).
//! 
//! `Hereditary` generates the boilerplate of trait implementations for the composited struct,
//! by [**Forwarding**](https://en.wikipedia.org/wiki/Forwarding_%28object-oriented_programming%29) the 
//! required methods that wrap the functionality already implemented in the contained instances referenced by its struct fields.
//! 
//! Currently, `Hereditary`support 2 kinds of delegation:
//! - **Partial Delegation**: By using the decorator attribute [`#[forward_trait(submember)]`](macro@forward_trait) on trait implementations.
//! - **Full Delegation**: By applying [`#[derive(Forwarding)]`](macro@Forwarding) on the composited struct, it derives the trait implementation
//!  on the struct field (designated by attribute `#[forward_derive(Trait)]`).
//! 
//! For creating the trait wrappers on subcomponent instances, it is necessary to generate the trait information
//! with the macro attribute [`trait_info`](macro@trait_info::trait_info). 
//! 
//! The process of incorporating reusable components by type composition, is performed in these three steps: 
//! 
//! ### 1. Trait Declaration
//! Before extending the functionality of traits in submembers, it's required to declare the macros that create 
//! the trait information syntax; such compile time information will be consumed by forwarding macros in later stages. 
//! It can be done by just inserting the `#[trait_info]` attribute on top of trait declartions:
//! ```
//! # extern crate hereditary as other_hereditary;
//! # mod hereditary {   pub use trait_info::trait_info as trait_info; }
//! mod animal
//! { 
//!     #[hereditary::trait_info]
//!     pub trait Cannis {
//!         fn bark(&self)-> String;
//!         fn sniff(&self)-> bool;
//!         fn roam(&mut self, distance:f64) -> f64;
//!     }
//!     // What `trait_info` does is declaring a macro with 'TraitInfo_' prefix, that injects 
//!     // the trait syntax structure as other forwarding macros would consume 
//!     // that compile time information by invoking the corresponging 'TraitInfo_' macros.
//!     // The resulting macro will be something like TraitInfo_Cannis(<inner params>)
//! 
//!     #[hereditary::trait_info]
//!     pub trait Bird {
//!         fn sing(&self) -> String;
//!         fn fly(&mut self, elevation:f64) -> f64;
//!     }
//! }
//! ```
//! ### 2. Compoment implementation
//! Create the basic components that would be reused in composite structs. Such
//! components provides a full implementation of the previously declared traits.
//! 
//! ```
//! # mod animal {
//! #    pub trait Cannis {
//! #        fn bark(&self)-> String;
//! #        fn sniff(&self)-> bool;
//! #        fn roam(&mut self, distance:f64) -> f64;     }
//! #    pub trait Bird {
//! #        fn sing(&self) -> String;
//! #        fn fly(&mut self, elevation:f64) -> f64; }
//! # }
//! struct Bulldog { position:f64 }
//! 
//! impl animal::Cannis for Bulldog {
//!     fn bark(&self)-> String {
//!         "Guau!".into()
//!     }
//! 
//!     fn sniff(&self)-> bool {true}
//! 
//!     fn roam(&mut self, distance:f64) -> f64 {
//!         self.position += distance;
//!         self.position
//!     }
//! }
//! 
//! // Bird implementation
//! struct Seagull {  elevation:f64 }
//! 
//! impl animal::Bird for Seagull 
//! {
//!     fn sing(&self) -> String {  "EEEYA!".into()  }
//!     fn fly(&mut self, elevation:f64) -> f64 {
//!         self.elevation += elevation;
//!         self.elevation
//!     }
//! }
//! 
//! ```
//! 
//! ### 3. Forwarding Traits in Composition.
//! 
//! By applying the procedural derive `Forwarding` macro (or the equivalent attribute macro `forward_trait` on partial trait implementations),
//! composite structs will obtain a trait adaptation by forwarding methods related to their subompoments.
//! ```
//! # extern crate hereditary as other_hereditary;
//! # mod hereditary {
//! #    pub use trait_info::trait_info as trait_info;
//! #    pub use forwarding::Forwarding as Forwarding;
//! #    pub use forwarding::forward_trait as forward_trait;
//! #    pub use forwarding::ForwardingTraitImplProcess as ForwardingTraitImplProcess;
//! #    pub use forwarding::ForwardingDeriveMemberProcess as ForwardingDeriveMemberProcess; }
//! # mod animal {
//! #    #[hereditary::trait_info]
//! #    pub trait Cannis {
//! #        fn bark(&self)-> String;
//! #        fn sniff(&self)-> bool; }
//! #    #[hereditary::trait_info]
//! #    pub trait Bird {
//! #        fn sing(&self) -> String;
//! #        fn fly(&mut self, elevation:f64) -> f64; }
//! # }
//! # struct Bulldog { position:f64 }
//! # impl animal::Cannis for Bulldog {
//! #    fn bark(&self)-> String {"Guau!".into()} fn sniff(&self)-> bool{true} }
//! # struct Seagull {  elevation:f64 }
//! # impl animal::Bird for Seagull {
//! #    fn sing(&self) -> String {  "EEEYA!".into()  } 
//! #    fn fly(&mut self, elevation:f64) -> f64 {
//! #        self.elevation += elevation;
//! #        self.elevation
//! #    }  }
//! // Heritance for an hybrid animal
//! #[derive(hereditary::Forwarding)]
//! struct KimeraSphinx 
//! {
//!     // notice that it needs referencing the trait from the animal module path
//!     #[forward_derive(animal::Cannis)] // full implementation of Cannis
//!     dogpart:Bulldog,
//!     birdpart:Seagull
//! }
//! # impl KimeraSphinx{
//! #    fn new() -> Self  {
//! #        Self { dogpart: Bulldog { position: 0f64 } , birdpart: Seagull { elevation: 0f64 } }
//! #    } }
//! 
//! // Sometimes a new custom behavior is needed.
//! // By combining new methods with existing functionality inherited from Bird
//! #[hereditary::forward_trait(birdpart)]
//! impl animal::Bird for KimeraSphinx
//! {
//!     fn sing(&self) -> String
//!     {
//!         use crate::animal::Cannis; // have to import the trait here
//!         // because is a dog, it barks
//!         self.dogpart.bark()
//!     }
//! }
//! 
//! fn main() {
//!     // Have to import the animal traits for accessing their methods here
//!     use crate::animal::Bird;
//!     use crate::animal::Cannis;
//! 
//!     // Instance kimera
//!     let mut kimera = KimeraSphinx::new();
//!     // A dogs that flies.
//!     assert_eq!(kimera.fly(50f64), 50f64);
//!     assert_eq!(kimera.bark(), kimera.sing()); // It barks instead of singing
//!     assert_eq!(kimera.sniff(), true);
//! }
//! ```
//! # Features
//! - Brings subtype polymorphism on composite structs with just one instruction, vesting
//! the new type with the same interface as its components.
//! - Re-use fields/method implementations from other types as subcomponents, without needing to repeately write wrapping code
//! that forwards the methods of those subcomponents.
//! - `Hereditary` tools are essentially *zero-cost abstractions*. They doesn't require runtime structures for holding trait type information. 
//! All the work it's done by macros and code generation.
//! - Embrace the [**New Type pattern**](https://www.lurklurk.org/effective-rust/newtype.html) effectively, but without the previous
//! awkward issues of having to re-implement the `inner-type` interfaces for the `new-type`. By using this technique Rust programmers
//! avoid the problems of incorporating new behaviour of existing foreign types, bypassing the 
//! [*Orphan Rule for traits*](https://doc.rust-lang.org/book/ch10-02-traits.html#implementing-a-trait-on-a-type).
//! 
//! 
//! # Limitations
//! - Because of the heavily usage of macros, code made with `Hereditary` would incurr in longer compilation processes.
//! - Sometimes, the traits information cannot be referenced by external modules, because `trait_info` generated macros
//! aren't imported automatically as same as their corresponding traits. That's why they need to be referenced with the full path 
//! in the forwarding attributes (`animal::Bird`), instead of just `Bird`. This is a known issue related with declarative macros and the scope rules for
//! their visibibility, as they have special needs when [exporting them as module symbols](https://danielkeep.github.io/tlborm/book/mbe-min-import-export.html).
//! 

/// Generates trait information syntax that can be injected as a macro invoke.
pub use trait_info::trait_info as trait_info;


/// Derive procedural macro for generating wrapping trait methods on 
/// struct members (designated by the `#[forward_derive]` attribute)
/// as they bring the required implementation of those traits.
/// 
/// This macro brings the full list of trait methods implemented for the subcomponent member, but 
/// it requires that the instanced traits were declared as [`object safe`](https://doc.rust-lang.org/reference/items/traits.html#object-safety)
/// exclusively: That means that they only contain dispatchable  methods and cannot include generics, constants or associated type items in
/// their original trait declaration.
/// 
/// Otherwise, If there is need to extend the functionality of more complex traits that aren't purely `object-safe` 
/// (with associated items and generic arguments), consider using [`forward_trait`](macro@forward_trait) instead.
/// 
/// Anyhow, instanced traits should incorporate their previously declared trait signature representation 
/// via [`trait_info`](macro@trait_info) macro.
/// 
/// ```rust
/// # extern crate hereditary as other_hereditary;
/// # mod hereditary {
/// #    pub use trait_info::trait_info as trait_info;
/// #    pub use forwarding::Forwarding as Forwarding;
/// #    pub use forwarding::forward_trait as forward_trait;
/// #    pub use forwarding::ForwardingTraitImplProcess as ForwardingTraitImplProcess;
/// #    pub use forwarding::ForwardingDeriveMemberProcess as ForwardingDeriveMemberProcess; }/// 
/// # #[hereditary::trait_info]
/// # trait IntefaceObj{}
/// # #[hereditary::trait_info]
/// # trait MyTrait2{}/// 
/// # #[hereditary::trait_info]
/// # trait NumOps{}/// 
/// # struct SubComponent{val:u32}
/// # struct SubComponent2{val:u32}
/// # struct SubComponent3{val:u32}
/// # impl IntefaceObj for SubComponent{}
/// # impl MyTrait2 for SubComponent{}
/// # impl NumOps for SubComponent2{}
/// // Forwarding allows to reusing components by vesting the composite struct with their interfaces
/// #[derive(hereditary::Forwarding)]
/// struct Composite
/// {
///     // Brings full implementation of IntefaceObj and MyTrait2 by wrapping
///     // forwarded methods on component1 instance.
///     #[forward_derive(IntefaceObj, MyTrait2)]
///     component1:SubComponent,
///     #[forward_derive(NumOps)]
///     component2:SubComponent2,
///     component3:SubComponent3,
/// }
/// ``` 
/// This derive macro works in collaboration with their associated inert attributes:
/// ### `forward_derive` 
/// Specifies a list of traits that will be extended from the submember struct field that implements them. (It needs to be located nearly above to 
/// the struct field).
/// ```text
/// #[forward_derive(Trait1, Trait2...<list of traits>)
/// field:Type
/// ```
/// ### `forwarding_mod_path` 
/// Optional attribute that can be used in cases when there is a need for re-exporting `hereditary` module items.
/// Because `Forwarding` generates declarative macros that depend on inner procedural macro functions
/// (as those are meant to be invoked within the generated code), it just keeps the references to those inner functions 
/// with their accessing path harcoded to the module `hereditary` by default.
/// 
/// In concrete details, the procedural derive macro `Forwarding` generates a declarative macro code that calls to the 
/// *`TraitInfo-like`* macro for obtaining the trait syntax information already generated by [`trait_info`](macro@trait_info), which
/// ends-up calling another inner procedural macro (`hereditary::ForwardingDeriveMemberProcess`) that processes the 
/// syntax information for the required trait and then constructs the corresponding forwarding methods linked to the
/// instancing field component.
/// 
/// So when `Forwarding` item needs to be re-exported from `hereditary` to another module namespace, for example
/// `new_module`, the inner procedural macros need to be re-exported as well. That means that the generated macro code
/// ends-up calling `new_module::ForwardingDeriveMemberProcess` instead of `hereditary::ForwardingDeriveMemberProcess`.
/// 
/// `forwarding_mod_path` allows to notify those changes to the procedural macro, by passing the new re-exporting module path
/// as a parameter for the code generative process.
/// ```text
/// #[derive(new_module::Forwarding)] // derive macro
/// #[forwarding_mod_path(new_path::new_module)] // attribute that tells the re-exporting module path
/// struct MyType{
/// ... fields
/// }
/// ```
/// 
pub use forwarding::Forwarding as Forwarding;

/// Attribute Macro that forwards the unimplemented trait methods on a [trait implementation item block](https://doc.rust-lang.org/reference/items/implementations.html#trait-implementations),
/// by wrapping instanced trait methods already implemented in the designated struct field.
/// 
/// ```text
/// // Attribute macro that forwards the methods from `field_name` instance:
/// #[forward_trait(field_name)]
/// impl Trait for Type
/// {
/// ...
/// // methods implemented in `field_name` instance will be inserted
/// // by preserving their original signature.
/// // fn method0(&self, param1, param2...) -> RetType
/// // {
/// //      self.field.method0(param1, param2, ...)
/// // }
/// }
/// ```
/// This attribute procedural macro can be used for extending the functionality of complex traits that aren't purely [`object safe`](https://doc.rust-lang.org/reference/items/traits.html#object-safety)
/// (those declared traits with associated items and generic arguments). Nevertheless, it could only extend dispatchable methods with 
/// `self` as receiver (reference and mutable ref).
/// 
/// Also this macro allows to combine custom method implementations with the existing methods from the subcomponent instance.
/// 
/// ```ignore
/// #[hereditary::forward_trait(birdpart)]
/// impl<'lifetime, TParam:boundtrait, Tparam2> animal::Bird
/// for KimeraSphinx<'lifetime, TParam:boundtrait>
/// {
///     type AssociatedType:String; // associated types
/// 
///     fn custom_method(param:u32, param2:&str) -> Result
///     {... }
/// 
///     // methods implemented from birdpart instance will be inserted here
/// }
/// ```
/// Anyhow, instanced traits should incorporate their previously declared trait signature representation 
/// via [`trait_info`](macro@trait_info) macro.
/// 
/// Also, `forward_trait` accepts an additional meta attribute that specifies a custom re-exporting module path:
/// ### `forwarding_mod_path` 
/// Optional attribute that can be used in cases when there is a need for re-exporting `hereditary` module items.
/// `forwarding_mod_path` allows to notify those changes to the procedural macro, by passing the new re-exporting module path
/// as a parameter for the code generative process.
/// ```text
/// #[forward_trait(field_name, forwarding_mod_path(new_path::new_module))]
/// impl Trait for Type{... }
/// ```
/// In concrete details, the procedural attribute macro `forward_trait` generates a declarative macro code that calls to the 
/// *`TraitInfo-like`* macro for obtaining the trait syntax information already generated by [`trait_info`](macro@trait_info), which
/// ends-up calling another inner procedural macro (`hereditary::ForwardingTraitImplProcess`) that processes the 
/// syntax information for the required trait and then constructs the corresponding forwarding methods linked to the
/// instancing field component.
pub use forwarding::forward_trait as forward_trait;


#[doc(hidden)]
pub use forwarding::ForwardingTraitImplProcess as ForwardingTraitImplProcess;
#[doc(hidden)]
pub use forwarding::ForwardingDeriveMemberProcess as ForwardingDeriveMemberProcess;


