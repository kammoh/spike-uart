use crate::{mmio_device::MmioDevice, ThisDeviceType};

type PluginType = dyn MmioPlugin<ThisDeviceType>;

pub trait MmioPlugin<T: MmioDevice> {}

impl<T: MmioDevice> dyn MmioPlugin<T> {
    fn new_c(args: Vec<String>) -> *mut libc::c_void {
        let boxed = Box::new(T::new(args).unwrap());
        Box::into_raw(boxed) as *mut libc::c_void
    }

    fn peek_from_c_mut(c_self: *mut libc::c_void) -> &'static mut T {
        let b = unsafe { Box::from_raw(c_self as *mut T) };
        Box::leak(b)
    }
    fn del_c(c_self: *mut libc::c_void) {
        unsafe {
            drop(Box::from_raw(c_self as *mut T));
        }
    }
    fn device_name() -> String {
        let type_name = String::from(std::any::type_name::<T>());
        type_name.split("::").last().unwrap().to_lowercase()
    }
}

#[repr(C)]
struct CMmioPlugin {
    alloc: extern "C" fn(*const libc::c_char) -> *mut libc::c_void,
    load: extern "C" fn(*mut libc::c_void, u64, libc::size_t, *mut libc::c_uchar) -> bool,
    store: extern "C" fn(*mut libc::c_void, u64, libc::size_t, *const libc::c_uchar) -> bool,
    dealloc: extern "C" fn(*mut libc::c_void),
}

extern "C" {
    fn register_mmio_plugin(name_cstr: *const libc::c_char, mmio_plugin: *const CMmioPlugin);
}

extern "C" fn alloc(c_args: *const libc::c_char) -> *mut libc::c_void {
    let args_string = unsafe { std::ffi::CStr::from_ptr(c_args).to_string_lossy().into_owned() };

    println!("[ALLOC] args={}", args_string);

    let args = args_string.split(',').map(|x| x.to_string()).collect::<Vec<_>>();

    PluginType::new_c(args)
}

extern "C" fn load(c_self: *mut libc::c_void, addr: u64, len: libc::size_t, c_bytes: *mut libc::c_uchar) -> bool {
    let bytes = unsafe { std::slice::from_raw_parts_mut(c_bytes, len) };
    let dev = PluginType::peek_from_c_mut(c_self);

    dev.load(addr, len)
        .and_then(|bs| {
            bytes.copy_from_slice(&bs);
            Ok(())
        })
        .is_ok()
}

extern "C" fn store(c_self: *mut libc::c_void, addr: u64, len: libc::size_t, c_bytes: *const libc::c_uchar) -> bool {
    let bytes = unsafe { std::slice::from_raw_parts(c_bytes, len) };
    let dev = PluginType::peek_from_c_mut(c_self);

    dev.store(addr, bytes).is_ok()
}

extern "C" fn dealloc(c_self: *mut libc::c_void) {
    println!("[DEALLOC]");
    PluginType::del_c(c_self);
}

#[ctor::ctor]
fn on_load() {
    let mmio_plugin = CMmioPlugin { alloc: alloc, load: load, store: store, dealloc: dealloc };

    let name = std::ffi::CString::new(PluginType::device_name() + "_plugin").unwrap();

    unsafe {
        register_mmio_plugin(name.as_ptr(), &mmio_plugin);
    }
}
