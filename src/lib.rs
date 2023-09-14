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

use jni::{
    objects::*,
    sys::{jint, jobject, jstring},
    JNIEnv,
};
use crate::file::read_config_from_json;

#[no_mangle]
pub unsafe extern "C" fn Java_pers_metaworm_RustJNI_init(env: JNIEnv, _class: JClass) {
    println!("rust-java-demo inited");
}

#[no_mangle]
pub unsafe extern "C" fn Java_pers_metaworm_RustJNI_config(
    mut env: JNIEnv,
    _class: JClass,
    config: JObject,
) {
    println!("Java_pers_metaworm_RustJNI_config");

    struct Config {
        text1: String,
        text2: String,
    }

    let text1_jstring: JString = env
        .get_field(&config, "text1", "Ljava/lang/String;")
        .unwrap()
        .l()
        .unwrap()
        .into();
    let text2_jstring: JString = env
        .get_field(&config, "text2", "Ljava/lang/String;")
        .unwrap()
        .l()
        .unwrap()
        .into();

    let text1: String = env.get_string(&text1_jstring).unwrap().into();
    let text2: String = env.get_string(&text2_jstring).unwrap().into();

    let config_struct = Config { text1, text2 };

    println!("input: {}", config_struct.text1);
    println!("input: {}", config_struct.text2);
}

fn json_login(json: String) {
    match read_config_from_json(json) {
        Ok(config) => {
            let config_i = config.clone();
            let server = config
                .server
                .clone().unwrap();
            for user in config_i {
                println!("login user: {:#?}", user);
                let mut client = SrunClient::new_from_user(&server, user)
                    .set_detect_ip(config.detect_ip)
                    .set_strict_bind(config.strict_bind)
                    .set_double_stack(config.double_stack);
                if let Some(n) = config.n {
                    client.set_n(n);
                }
                if let Some(utype) = config.utype {
                    client.set_type(utype);
                }
                if let Some(acid) = config.acid {
                    client.set_acid(acid);
                }
                if let Some(ref os) = config.os {
                    client.set_os(os);
                }
                if let Some(ref name) = config.name {
                    client.set_name(name);
                }
                if let Some(retry_delay) = config.retry_delay {
                    client.set_retry_delay(retry_delay);
                }
                if let Some(retry_times) = config.retry_times {
                    client.set_retry_times(retry_times);
                }

                if let Err(e) = client.login() {
                    println!("login error: {}", e);
                }
            }
        }
        Err(e) => {
            println!("read config file error: {}", e);
        }
    }
}