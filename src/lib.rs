pub mod error;
mod project;

use leptos::wasm_bindgen::prelude::*;
use leptos::web_sys::*;
use leptos::*;

pub use error::*;
use crate::project::Project;

#[wasm_bindgen(js_name=LogicXContext)]
pub struct LogicX {
    project: Project
}

#[wasm_bindgen(js_class=LogicXContext)]
impl LogicX {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            project: Project::empty()
        }
    }

    #[wasm_bindgen]
    pub fn mount(&self, root: HtmlDivElement) {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        root.set_inner_html("");

        mount_to(root.unchecked_into(), move || view! {
            <div>
                {"Hello World"}
            </div>
        });
    }


    #[wasm_bindgen(js_name=getData)]
    pub fn get_data(&self) -> String {
        serde_json::to_string_pretty(&self.project)
            .unwrap_throw()
    }

    #[wasm_bindgen(js_name=setData)]
    pub fn set_data(&mut self, data: String, clear: bool) {
        if let Ok(project) = serde_json::from_str(data.as_str()) {
            if clear {
                self.clear();
            }

            self.project = project;
        }
    }

    #[wasm_bindgen(js_name=clear)]
    pub fn clear(&mut self) {
        self.project = Project::empty()
    }
}

