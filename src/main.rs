#[macro_use]
extern crate mozjs;

use mozjs::rust::{self, Runtime, SIMPLE_GLOBAL_CLASS};
use mozjs::jsval::*;
use mozjs::jsapi::*;

thread_local!(static RT: (Heap<*mut JSObject>, Runtime) = unsafe {
    let rt = Runtime::new().unwrap();

    let glob: Heap<*mut JSObject> = Heap::default();
    glob.set(JS_NewGlobalObject(
        rt.cx(),
        &SIMPLE_GLOBAL_CLASS,
        ::std::ptr::null_mut(),
        OnNewGlobalHookOption::DontFireOnNewGlobalHook,
        &CompartmentOptions::default()
    ));

    let _ac = JSAutoCompartment::new(rt.cx(), glob.get());

    JS_InitReflectParse(rt.cx(), glob.handle());
    JS_FireOnNewGlobalObject(rt.cx(), glob.handle());

    (glob, rt)
});

fn main() {
    RT.with(|&(ref glob, ref rt)| unsafe {
        let _ac = JSAutoCompartment::new(rt.cx(), glob.get());

        rooted!(in(rt.cx()) let mut rval = UndefinedValue());

        rt.evaluate_script(
            rust::Handle::from_raw(glob.handle()),
            "'1 + 2 = ' + (1 + 2)",
            "nofile.js",
            1,
            rval.handle_mut(),
        ).unwrap();

        let s = rval.get().to_string();
        let chars = JS_EncodeStringToUTF8(rt.cx(), Handle::from_marked_location(&s));
        let chars = ::std::ffi::CStr::from_ptr(chars);

        println!("result: {}", chars.to_string_lossy());
    });
}
