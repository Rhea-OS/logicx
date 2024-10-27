pub mod error;

use leptos::*;
use leptos::wasm_bindgen::prelude::*;
use leptos::web_sys::HtmlDivElement;
use error::*;

#[wasm_bindgen]
pub fn mount(root: HtmlDivElement, ) {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    root.set_inner_html("");
    
    mount_to(root.unchecked_into(), move || view! {
        <div>
            {"Hello World"}
        </div>
    });
}