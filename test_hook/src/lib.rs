pub use test_hook_macro::{test, test_hook};

mod impl_;

#[doc(hidden)]
pub use impl_::{TestHook as __TestHook, call_test_hook as __call_test_hook};
