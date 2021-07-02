use std::ffi::{CStr, CString};

pub extern "C" fn test_attach_function(_payload: *mut std::ffi::c_void) -> i32 {
    let payload: &mut u64 = &mut unsafe { *(_payload as *mut u64) };
    let container = if payload == &1u64 {
        "Inside  Container"
    } else {
        "Outside Container"
    };
    let contents = std::fs::read_to_string("/etc/group")
        .expect("Something went wrong reading the file");
    println!(
        "[{}] read {} bytes, pid: {}",
        container,
        contents.len(),
        std::process::id()
    );
    0
}

fn main() {
    let name = CString::new("playground").unwrap();
    // let configpath = CString::new("$lxc.lxcpath").unwrap().as_ptr();
    let configpath = std::ptr::null();
    let c = unsafe { lxc_sys2::lxc_container_new(name.as_ptr(), configpath) };
    if c.is_null() {
        panic!("Error getting container");
    }
    let c_state = unsafe { CStr::from_ptr(((*c).state)(c)) };
    println!("Container {:?} is in state: {:?}", name, c_state);
    let c_defined = unsafe { ((*c).is_defined)(c) };
    println!("Container {:?} is defined: {:?}", name, c_defined);

    let lxc_attach_options = lxc_sys2::lxc_attach_options_t::default();
    let mut pid = 0;
    let mut payload: u64 = 1u64;

    // Will fail when container is not running, returning -1
    let ret = unsafe {
        ((*c).attach)(
            c,
            test_attach_function,
            &mut payload as *mut u64 as *mut std::ffi::c_void,
            &lxc_attach_options,
            &mut pid,
        )
    };
    println!("pid: {}, ret: {}", pid, ret);

    payload = 2;
    test_attach_function(&mut payload as *mut u64 as *mut std::ffi::c_void);
}
