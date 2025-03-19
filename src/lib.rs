#![no_std]
extern crate self as init_hook;

#[doc(hidden)]
#[path = "private.rs"]
pub mod __private;

pub use init_hook_macros::call_on_init;

#[doc(hidden)]
pub use linkme;

/// Call all functions registered by [call_on_init]
///
/// # Example
///
/// ```
/// use std::sync::atomic::{AtomicUsize, Ordering};
/// static COUNTER: AtomicUsize = AtomicUsize::new(0);
///
/// #[init_hook::call_on_init]
/// unsafe fn init_once_unchecked() {
///     COUNTER.fetch_add(1, Ordering::Release);
/// }
///
/// #[init_hook::call_on_init]
/// fn init_once() {
///     COUNTER.fetch_add(1, Ordering::Release);
/// }
///
/// fn main() {
///    init_hook::init!();
///    assert_eq!(COUNTER.load(Ordering::Acquire), 2);
/// }
/// ```
///
/// # Panic
///
/// If init isn't used in main exactly once, `init_hook` will detect this and panic pre-main
///
/// ```should_panic
/// use std::sync::atomic::{AtomicBool, Ordering};
/// static INIT_CALLED: AtomicBool = AtomicBool::new(false);
///
/// #[init_hook::call_on_init]
/// fn init() {
///     INIT_CALLED.store(true, Ordering::Release);
/// }
///
/// // This will panic with "`init_hook::init` must be used within the root main function"
/// fn main() {
///    let _init_called = INIT_CALLED.load(Ordering::Acquire);
/// }
/// ```
#[macro_export]
macro_rules! init {
    () => {
        #[cfg(not(test))]
        {
            if module_path!().contains("::") {
                panic!("#[init_hook::init] can only be used in the crate root");
            }
        }

        #[init_hook::linkme::distributed_slice(init_hook::__private::INIT)]
        #[linkme(crate = init_hook::linkme)]
        static INIT_CONFIGURED: bool = true;

        unsafe {
            init_hook::__private::call_init_fns();
        }
    };
}
