use libc;

mod canvas;
pub use self::canvas::*;
mod image;
pub use self::image::*;

type EMCallbackArg = extern "C" fn(*mut libc::c_void);

#[repr(C)]
#[derive(Debug)]
pub struct EmscriptenMouseEvent {
    pub timestamp: libc::c_double,
    pub screen_x: libc::c_long,
    pub screen_y: libc::c_long,
    pub client_x: libc::c_long,
    pub client_y: libc::c_long,
    pub ctrl_key: libc::c_int,
    pub shift_key: libc::c_int,
    pub alt_key: libc::c_int,
    pub meta_key: libc::c_int,
    pub button: libc::c_ushort,
    pub buttons: libc::c_ushort,
    pub movement_x: libc::c_long,
    pub movement_y: libc::c_long,
    pub target_x: libc::c_long,
    pub target_y: libc::c_long,
    pub canvas_x: libc::c_long,
    pub canvas_y: libc::c_long,
    padding: libc::c_long,
}

type ENMouseCallback = extern "C" fn (ty: libc::c_int, event: *const EmscriptenMouseEvent, user_data: *mut libc::c_void) -> libc::c_int;

#[link_args = "--js-library js/lib.js --pre-js js/pre.js -s RESERVED_FUNCTION_POINTERS=20"]
extern "C" {
    fn emscripten_set_main_loop_arg(func: EMCallbackArg, arg: *mut libc::c_void, fps: libc::c_int, simulate_infinite_loop: libc::c_int);

    fn get_body_element() -> libc::c_int;
    fn get_element_by_id(id: *const libc::c_char, id_len: libc::c_int) -> libc::c_int;
    fn create_element(tag: *const libc::c_char, tag_len: libc::c_int) -> libc::c_int;
    fn drop_ref(ref_id: libc::c_int);
    fn clone_ref(ref_id: libc::c_int) -> libc::c_int;

    fn append_child(ref_a: libc::c_int, ref_b: libc::c_int);
    fn js_set_prop_str(eref: libc::c_int, key: *const libc::c_char, key_len: libc::c_int, val: *const libc::c_char, val_len: libc::c_int);
    fn js_set_prop_int(eref: libc::c_int, key: *const libc::c_char, key_len: libc::c_int, val: libc::c_int);
    fn js_get_prop_int(eref: libc::c_int, key: *const libc::c_char, key_len: libc::c_int) -> libc::c_int;
    fn js_set_prop_ref(eref: libc::c_int, key: *const libc::c_char, key_len: libc::c_int, vref: libc::c_int);
    fn js_set_prop_float(eref: libc::c_int, key: *const libc::c_char, key_len: libc::c_int, val: libc::c_double);

    fn poll_event() -> libc::c_int;
    fn emscripten_get_mouse_status_fixed(state: *mut EmscriptenMouseEvent) -> libc::c_int;
    fn emscripten_set_mousemove_callback(target: *const libc::c_char, userdata: *mut libc::c_void, use_capture: libc::c_int, callback: ENMouseCallback);
    fn emscripten_set_mousedown_callback(target: *const libc::c_char, userdata: *mut libc::c_void, use_capture: libc::c_int, callback: ENMouseCallback);
    fn emscripten_set_mouseup_callback(target: *const libc::c_char, userdata: *mut libc::c_void, use_capture: libc::c_int, callback: ENMouseCallback);
}

pub fn hook_events(target: &str) {
    unsafe {
        let target = ::std::ffi::CString::new(target).unwrap();
        emscripten_set_mousemove_callback(target.as_ptr(), ::std::ptr::null_mut(), 1, dummy_cb);
        emscripten_set_mousedown_callback(target.as_ptr(), ::std::ptr::null_mut(), 1, dummy_cb);
        emscripten_set_mouseup_callback(target.as_ptr(), ::std::ptr::null_mut(), 1, dummy_cb);
    }
}

extern "C" fn dummy_cb(_ty: libc::c_int, _event: *const EmscriptenMouseEvent, _user_data: *mut libc::c_void) -> libc::c_int {
    1
}

pub fn main_loop<F: FnMut() + 'static>(func: F) {
    unsafe {
        let fpointer = Box::new(func);
        emscripten_set_main_loop_arg(c_main_loop::<F>, Box::into_raw(fpointer) as *mut _, 0, 1);
    }
}

extern "C" fn c_main_loop<F: FnMut()>(arg: *mut libc::c_void) {
    let func: *mut F = arg as *mut _;
    unsafe {
        (*func)()
    }
}

pub fn get_mouse_state() -> EmscriptenMouseEvent {
    unsafe {
        let mut evt = ::std::mem::zeroed();
        emscripten_get_mouse_status_fixed(&mut evt);
        evt
    }
}

pub struct KeyEvent {
    pub up: bool,
    pub key: i32,
}

pub fn poll_key_event() -> Option<KeyEvent> {
    unsafe {
        let evt = poll_event();
        if evt == 0 {
            None
        } else {
            Some(KeyEvent {
                up: (evt >> 16) == 2,
                key: evt & 0xFFFF,
            })
        }
    }
}

fn set_prop_str(eref: libc::c_int, key: &str, val: &str) {
    unsafe {
        js_set_prop_str(
            eref,
            key.as_ptr() as *const _, key.as_bytes().len() as _,
            val.as_ptr() as *const _, val.as_bytes().len() as _
        );
    }
}

fn set_prop_float(eref: libc::c_int, key: &str, val: f64) {
    unsafe {
        js_set_prop_float(
            eref,
            key.as_ptr() as *const _, key.as_bytes().len() as _,
            val as _,
        );
    }
}

fn set_prop_int(eref: libc::c_int, key: &str, val: i32) {
    unsafe {
        js_set_prop_int(
            eref,
            key.as_ptr() as *const _, key.as_bytes().len() as _,
            val as _,
        );
    }
}

fn get_prop_int(eref: libc::c_int, key: &str) -> i32 {
    unsafe {
        js_get_prop_int(
            eref,
            key.as_ptr() as *const _, key.as_bytes().len() as _,
        ) as _
    }
}

fn set_prop_ref(eref: libc::c_int, key: &str, vref: libc::c_int) {
    unsafe {
        js_set_prop_ref(
            eref,
            key.as_ptr() as *const _, key.as_bytes().len() as _,
            vref,
        );
    }
}

fn get_element_raw(id: &str) -> libc::c_int {
    unsafe {
        get_element_by_id(id.as_ptr() as *const _, id.as_bytes().len() as _)
    }
}

fn make_element_raw(tag: &str) -> libc::c_int {
    unsafe {
        create_element(tag.as_ptr() as *const _, tag.as_bytes().len() as _)
    }
}

struct UnknownElement(libc::c_int);

unsafe impl JSObject for UnknownElement {
    unsafe fn get_internal(&self) -> libc::c_int {
        self.0
    }
}

impl JSElement for UnknownElement {
}

pub fn get_body() -> impl JSElement {
    unsafe { UnknownElement(get_body_element()) }
}

pub fn element_by_id(id: &str) -> impl JSElement {
    UnknownElement(get_element_raw(id))
}

pub fn make_element(tag: &str) -> impl JSElement {
    UnknownElement(make_element_raw(tag))
}

pub unsafe trait JSObject {
    unsafe fn get_internal(&self) -> libc::c_int;
}

pub trait JSElement: JSObject {

    fn append_child<E: JSElement>(&self, other: &E) {
        unsafe {
            append_child(self.get_internal(), other.get_internal())
        }
    }

    fn get_client_width(&self) -> i32 {
        unsafe {
            get_prop_int(self.get_internal(), "clientWidth")
        }
    }

    fn get_client_height(&self) -> i32 {
        unsafe {
            get_prop_int(self.get_internal(), "clientHeight")
        }
    }
}

