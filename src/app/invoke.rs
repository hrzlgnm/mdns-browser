use tauri_sys::core::invoke;

use crate::log_fn;

pub async fn invoke_no_args(cmd: impl Into<String>) {
    let cmd = cmd.into();
    log_fn!(format!("invoke_no_args(`{}`)", cmd), {
        let _ = invoke::<()>(cmd.as_str(), &()).await;
    })
}
