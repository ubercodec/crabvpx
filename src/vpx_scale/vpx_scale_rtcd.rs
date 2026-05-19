use std::sync::Once;

unsafe extern "C" {
    fn arm_cpu_caps() -> ::core::ffi::c_int;
}

static INIT: Once = Once::new();

fn setup_rtcd_internal() {
    // Safety: Calling FFI function arm_cpu_caps.
    unsafe {
        let _flags = arm_cpu_caps();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn vpx_scale_rtcd() {
    INIT.call_once(|| {
        setup_rtcd_internal();
    });
}
