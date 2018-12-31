#[macro_use]
extern crate mozjs;

use mozjs::{
    rust::{
        *,
        jsapi_wrapped::*,
    },
    jsapi::{
        Heap,
        JSObject,
        JS_NewGlobalObject,
        OnNewGlobalHookOption,
        CompartmentOptions,
        JSAutoCompartment,
    },
    jsval:: {
        UndefinedValue,
    },
};

static JAVASCRIPT: &'static str = r#"
function *fibonacci() {
  let cur = 1
  let nxt = 1
  for (;;) {
    yield cur
    ;[cur, nxt] = [nxt, cur + nxt]
  }
}

(() => {
  const out = ['fibonacci:']
  const it = fibonacci()
  for (let i = 0; i < 10; ++i) {
    out.push(it.next().value)
  }
  return out.join(' ')
})()
"#;

fn main() {
    let rt = Runtime::new().unwrap();

    let glob: Heap<*mut JSObject> = Heap::default();
    let glob_handle = unsafe { Handle::from_raw(glob.handle()) };

    let _ac = unsafe {
        glob.set(JS_NewGlobalObject(
            rt.cx(),
            &SIMPLE_GLOBAL_CLASS,
            ::std::ptr::null_mut(),
            OnNewGlobalHookOption::DontFireOnNewGlobalHook,
            &CompartmentOptions::default()
        ));

        let _ac = JSAutoCompartment::new(rt.cx(), glob.get());

        JS_InitReflectParse(rt.cx(), glob_handle);
        JS_FireOnNewGlobalObject(rt.cx(), glob_handle);

        _ac
    };

    rooted!(in(rt.cx()) let mut rval = UndefinedValue());

    rt.evaluate_script(
        glob_handle,
        JAVASCRIPT,
        "nofile.js",
        1,
        rval.handle_mut(),
    ).unwrap();

    let chars = unsafe {
        let s = rval.get().to_string();
        let s = JS_EncodeStringToUTF8(
            rt.cx(), Handle::from_marked_location(&s));
        let s = ::std::ffi::CStr::from_ptr(s);
        String::from(s.to_string_lossy())
    };

    println!("result: {}", chars);
}
