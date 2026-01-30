use jni::objects::{JValue, JString};
use jni::JNIEnv;
use crate::{
    android::main::java_vm,
    il2cpp::{ext::StringExt, hook::UnityEngine_CoreModule::Application}
};

pub fn open_app_or_fallback(package_name: &str, fallback_url: &str) {
    let vm = match java_vm() {
        Some(v) => v,
        None => return,
    };

    let mut env = match vm.attach_current_thread() {
        Ok(e) => e,
        Err(_) => return,
    };

    let try_open = |env: &mut JNIEnv| -> jni::errors::Result<()> {
        let unity_player = env.find_class("com/unity3d/player/UnityPlayer")?;
        let activity = env.get_static_field(unity_player, "currentActivity", "Landroid/app/Activity;")?.l()?;

        let intent_class = env.find_class("android/content/Intent")?;
        let action_main = env.new_string("android.intent.action.MAIN")?;
        let category_launcher = env.new_string("android.intent.category.LAUNCHER")?;

        let intent_obj = env.new_object(
            &intent_class,
            "(Ljava/lang/String;)V",
            &[JValue::from(&action_main)],
        )?;

        env.call_method(&intent_obj, "addCategory", "(Ljava/lang/String;)Landroid/content/Intent;", &[JValue::from(&category_launcher)])?;

        let pkg_name_java = env.new_string(package_name)?;
        env.call_method(&intent_obj, "setPackage", "(Ljava/lang/String;)Landroid/content/Intent;", &[JValue::from(&pkg_name_java)])?;

        env.call_method(&intent_obj, "setFlags", "(I)Landroid/content/Intent;", &[JValue::Int(0x10000000)])?;

        env.call_method(&activity, "startActivity", "(Landroid/content/Intent;)V", &[JValue::from(&intent_obj)])?;
        Ok(())
    };

    if let Err(e) = try_open(&mut env) {
        if env.exception_check().unwrap_or(false) {
            if let Ok(ex) = env.exception_occurred() {
                let _ = env.exception_clear();

                if let Ok(msg_obj) = env.call_method(ex, "toString", "()Ljava/lang/String;", &[]) {
                    let msg_jstr: JString = msg_obj.l().unwrap().into();
                    let msg_rust: String = env.get_string(&msg_jstr).unwrap().into();
                    info!("open_app_or_fallback: Java Exception: {}", msg_rust);
                }
            }
        }
        
        info!("open_app_or_fallback: Intent failed ({}), falling back to OpenURL", e);

        let url_ptr = fallback_url.to_string().to_il2cpp_string();
        Application::OpenURL(url_ptr);
    }
}

pub fn get_device_api_level(env: *mut jni::sys::JNIEnv) -> i32 {
    let mut env = unsafe { JNIEnv::from_raw(env).unwrap() };
    env.get_static_field("android/os/Build$VERSION", "SDK_INT", "I")
        .unwrap()
        .i()
        .unwrap()
}

