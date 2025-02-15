use web_sys::{console};
pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub async fn sleep(millis: i32) {
    let mut cb = |resolve: js_sys::Function, _reject: js_sys::Function| {
        window()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, millis)
            .unwrap();
    };
    let p = js_sys::Promise::new(&mut cb);
    wasm_bindgen_futures::JsFuture::from(p).await.unwrap();
}

pub fn print(s: &str) {
    console::log_1(&s.into());
}
