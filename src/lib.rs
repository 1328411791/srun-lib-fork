mod user;

use std::ffi::c_char;
pub use user::User;

mod srun;
pub use srun::*;

mod xencode;
pub use xencode::param_i;

mod file;
pub use file::read_config_from_file;

mod utils;
pub use utils::select_ip;

#[cfg(feature = "ureq")]
mod http_client;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use jni::objects::*;
use jni::sys::{jint, jobject, jstring};
use jni::JNIEnv;



#[no_mangle]
pub unsafe extern "C" fn Java_pers_metaworm_RustJNI_init(env: JNIEnv, _class: JClass) {
    println!("rust-java-demo inited");
}

#[no_mangle]
pub unsafe extern "C" fn Java_pers_metaworm_RustJNI_config(mut env: JNIEnv, _class: JClass
                                                           , config:JObject,text1:JString)->jobject {
    println!("Java_pers_metaworm_RustJNI_config");

    struct Config {
        text1:String,
        text2:String
    }

    let text1_jstring: JString = env.get_field(config, "text1", "Ljava/lang/String;").unwrap().l().unwrap();
    let text2_jstring: JString = env.get_field(config, "text2", "Ljava/lang/String;").unwrap().l().unwrap();

    let text1: String = env.get_string(&text1_jstring).unwrap().into();
    let text2: String = env.get_string(&text2_jstring).unwrap().into();

    let config_struct = Config { text1, text2 };

    println!("input: {}", config_struct.text1);
    println!("input: {}", config_struct.text2);


}

