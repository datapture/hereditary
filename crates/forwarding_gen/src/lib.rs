/*
Hereditary
Autors: Francisco Leon <https://github.com/superoptimo>
License Apache-2.0
*/

mod method_member_adapter;
mod forwarding_trait_impl_receiver;
mod forwarding_trait_impl_input;
mod forwarding_derive_member_receiver;
mod forwarding_derive_input;

pub use forwarding_trait_impl_receiver::ForwardingTraitImplReceiver as ForwardingTraitImplReceiver;
pub use forwarding_derive_member_receiver::ForwardingDeriveMemberReceiver as ForwardingDeriveMemberReceiver;
pub use forwarding_derive_input::{ForwardingDeriveInput, ForwardingDeriveMemberTask};
pub use forwarding_trait_impl_input::ForwardingTraitImplInput as ForwardingTraitImplInput;
