use linkme::distributed_slice;

#[distributed_slice]
pub static INIT: [bool];

#[cfg(not(test))]
#[ctor::ctor]
fn assert_configered() {
    if INIT.len().gt(&1) {
        panic!("`init_hook::init` can only be used once");
    } else if INIT.ne(&[true]) {
        panic!("`init_hook::init` must be used within the root main function");
    }
}

#[distributed_slice]
pub static UNSAFE_INIT_FNS: [unsafe fn()];

#[distributed_slice]
pub static INIT_FNS: [fn()];

pub unsafe fn call_init_fns() {
    for unsafe_init_fn in UNSAFE_INIT_FNS.iter() {
        unsafe {
            unsafe_init_fn();
        }
    }

    for init_fn in INIT_FNS.iter() {
        init_fn()
    }
}
