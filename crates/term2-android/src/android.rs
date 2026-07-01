use jni::objects::JClass;
use jni::signature::JavaType;
use jni::sys::jstring;
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_com_term2_RustBridge_hello<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jstring {
    let message = env
        .new_string("Hello from Term2 Rust core")
        .expect("failed to allocate Java string");
    message.into_raw()
}
