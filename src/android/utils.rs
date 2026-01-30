use jni::objects::{JValue, JString};
use jni::JNIEnv;
use crate::{
    android::main::java_vm,
    il2cpp::{ext::StringExt, hook::UnityEngine_CoreModule::Application}
};

pub fn open_app_or_fallback(app_uri: &str, fallback_url: &str) {
    // let app_uri = app_uri.to_string();
    // let fallback_url = fallback_url.to_string();

    // Thread::main_thread().schedule(move || {
    let vm = match java_vm() {
        Some(v) => v,
        None => {
            info!("open_app_or_fallback: No JVM found");
            return;
        }
    };

    let mut env = match vm.attach_current_thread() {
        Ok(e) => e,
        Err(e) => {
            info!("open_app_or_fallback: Failed to attach thread: {}", e);
            return;
        }
    };

    let try_open = |env: &mut JNIEnv| -> jni::errors::Result<()> {
        let unity_player = env.find_class("com/unity3d/player/UnityPlayer")?;
        let activity = env.get_static_field(unity_player, "currentActivity", "Landroid/app/Activity;")?.l()?;

        let uri_class = env.find_class("android/net/Uri")?;

        let app_uri_java: JString = env.new_string(app_uri)?;

        let uri_obj = env.call_static_method(
            uri_class,
            "parse",
            "(Ljava/lang/String;)Landroid/net/Uri;",
            &[JValue::from(&app_uri_java)],
        )?.l()?;

        let intent_class = env.find_class("android/content/Intent")?;
        let action_view: JString = env.new_string("android.intent.action.VIEW")?;

        let intent_obj = env.new_object(
            &intent_class,
            "(Ljava/lang/String;Landroid/net/Uri;)V",
            &[JValue::from(&action_view), JValue::from(&uri_obj)],
        )?;

        env.call_method(&activity, "startActivity", "(Landroid/content/Intent;)V", &[JValue::from(&intent_obj)])?;
        Ok(())
    };

    if let Err(e) = try_open(&mut env) {
        if env.exception_check().unwrap_or(false) {
            let _ = env.exception_clear();
            info!("open_app_or_fallback: Java Exception cleared. App likely not installed.");
        }
        info!("open_app_or_fallback: Intent failed ({}), falling back to OpenURL", e);
        let url_ptr = fallback_url.to_string().to_il2cpp_string();
        Application::OpenURL(url_ptr);
    }
    // });
}

pub fn get_device_api_level(env: *mut jni::sys::JNIEnv) -> i32 {
    let mut env = unsafe { JNIEnv::from_raw(env).unwrap() };
    env.get_static_field("android/os/Build$VERSION", "SDK_INT", "I")
        .unwrap()
        .i()
        .unwrap()
}

