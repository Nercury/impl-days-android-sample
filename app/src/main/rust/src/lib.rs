extern crate jni;

use jni::JNIEnv;
use jni::objects::JObject;
use jni::sys::jstring;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_example_hellojni_HelloJni_stringFromJNI(
    env: JNIEnv,
    _this: JObject
) -> jstring {
    env.new_string("Hello from Rust").expect("Could not create string").into_inner()
}

#[link(name = "crtbegin_so")]
mod crtbegin {
}

#[link(name = "crtend_so")]
mod crtend {
}