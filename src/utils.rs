// SPDX-FileCopyrightText: © 2020 Matt Williams <matt@milliams.com>
// SPDX-License-Identifier: MIT

use wasm_bindgen::prelude::*;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn get_local_timezone() -> Option<chrono_tz::Tz> {
    let options = js_sys::Intl::DateTimeFormat::new(&js_sys::Array::new(), &js_sys::Object::new()).resolved_options();
    let tz2 = js_sys::Reflect::get(&options, &JsValue::from("timeZone")).ok()?.as_string()?;
    tz2.parse().ok()
}
