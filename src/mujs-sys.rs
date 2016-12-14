extern crate libc;

use std::ffi::{CString, CStr};
use std::ptr;

#[repr(C)]
enum JsFlags {
    JsStrict = 1,
}

pub enum JsState {}
pub type JsStateRef = *mut JsState;

#[link(name = "mujs")]
extern "C" {
    // state
    fn js_newstate(alloc: Option<extern "C" fn(*mut libc::c_void,
                                               *mut libc::c_void,
                                               libc::c_int)>,
                   actx: *mut libc::c_void,
                   flags: libc::c_int)
                   -> JsStateRef;
    fn js_freestate(state: JsStateRef);

    // eval
    fn js_dostring(state: JsStateRef, source: *const libc::c_char);

    // new
    fn js_newcfunction(state: JsStateRef,
                       func: Option<extern "C" fn(state: JsStateRef)>,
                       name: *const libc::c_char,
                       arity: libc::c_int);
    fn js_newobject(state: JsStateRef);

    // set
    fn js_setglobal(state: JsStateRef, name: *const libc::c_char);
    fn js_setproperty(state: JsStateRef, num: libc::c_int, name: *const libc::c_char);

    // push
    fn js_pushundefined(state: JsStateRef);
    fn js_pushnumber(state: JsStateRef, number: libc::c_double);
    fn js_pushboolean(state: JsStateRef, number: libc::c_int);

    // args
    fn js_tostring(state: JsStateRef, index: libc::c_int) -> *const libc::c_char;
    fn js_gettop(state: JsStateRef) -> libc::c_int;
}

// extra stuff
pub struct JsContext(JsStateRef, bool);

impl JsContext {
    pub fn new() -> JsContext {
        unsafe {
            let stateptr = js_newstate(None, ptr::null_mut(), JsFlags::JsStrict as libc::c_int);
            JsContext(stateptr, true)
        }
    }

    pub fn shadow(state: JsStateRef) -> JsContext {
        JsContext(state, false)
    }

    pub fn run(&mut self, source: &str) {
        let result = CString::new(source).unwrap();

        unsafe {
            js_dostring(self.0, result.as_ptr());
        }
    }

    pub fn register(&mut self,
                    callback: extern "C" fn(state: JsStateRef),
                    name: &str,
                    arity: usize) {
        let c_name = CString::new(name).unwrap();

        unsafe {
            js_newcfunction(self.0,
                            Some(callback),
                            c_name.as_ptr(),
                            arity as libc::c_int);
            js_setglobal(self.0, c_name.as_ptr());
        }
    }

    pub fn tostring<'a>(&mut self, index: usize) -> &'a CStr {
        unsafe {
            let c_str = js_tostring(self.0, index as libc::c_int);
            CStr::from_ptr(c_str)
        }
    }

    pub fn gettop(&mut self) -> usize {
        unsafe { js_gettop(self.0) as usize }
    }

    pub fn push_undefined(&mut self) {
        unsafe {
            js_pushundefined(self.0);
        }
    }

    pub fn pushnumber(&mut self, num: f64) {
        unsafe {
            js_pushnumber(self.0, num);
        }
    }

    pub fn pushboolean(&mut self, boolean: bool) {
        unsafe {
            js_pushboolean(self.0, boolean as i32);
        }
    }

    pub fn newobject(&mut self) {
        unsafe {
            js_newobject(self.0);
        }
    }

    pub fn setproperty(&mut self, num: i32, name: &str) {
        let c_name = CString::new(name).unwrap();

        unsafe {
            js_setproperty(self.0, num, c_name.as_ptr());
        }
    }
}

impl Drop for JsContext {
    fn drop(&mut self) {
        if self.1 {
            unsafe {
                js_freestate(self.0);
            }
        }
    }
}
