pub mod error;
mod project;
mod circuit;
mod component;
mod wire;
mod drag;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use leptos::wasm_bindgen::prelude::*;
use leptos::web_sys::*;
use leptos::*;
use leptos::prelude::*;
use leptos::svg::Svg;
pub use error::*;
use crate::project::{Coord, InstanceId, Project, Terminal};

#[wasm_bindgen(js_name=LogicXContext)]
pub struct LogicX {
    project: RwSignal<Project>,
    state: RwSignal<State>
}

#[wasm_bindgen(js_class=LogicXContext)]
impl LogicX {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            project: RwSignal::new(Project::empty()),
            state: RwSignal::new(State {
                grid_scale: 35.0,
                viewport: NodeRef::new(),
                snap: true,
                scroll: (0.0, 0.0).into(),
                start_connect_wire: None
            })
        }
    }

    #[wasm_bindgen]
    pub fn mount(&self, root: HtmlDivElement) {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        root.set_inner_html("");

        let project = self.project.clone();
        let state = self.state.clone();

        mount_to(root.unchecked_into(), move || view!(<ContextProvider cx=state.clone()>
            <ContextProvider cx=project.clone()>
                <Project />
            </ContextProvider>
        </ContextProvider>)).forget();
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
        self.project.set(Project::empty());
    }
}

#[derive(Clone)]
pub(crate) struct WireConnectStart {
    pub(crate) from: InstanceId,
    pub(crate) start_terminal: Terminal,
    pub(crate) to: Coord
}

#[derive(Clone)]
pub struct State {
    pub scroll: Coord,
    pub grid_scale: f64,
    pub snap: bool,

    pub start_connect_wire: Option<WireConnectStart>,

    pub viewport: NodeRef<Svg>
}

impl State {
    pub fn viewport(&self) -> Coord {
        self.viewport
            .with(|el| el
            .as_ref()
            .map(|el| el.get_bounding_client_rect())
            .map(|rect| Coord(rect.x(), rect.y()))
            .unwrap_or(Coord(0.0, 0.0)))
    }
}

#[component]
pub fn context_provider<T: Send + Sync + 'static>(cx: T, children: Children) -> impl IntoView {
    provide_context(cx);

    children()
}


pub trait Dud {
    /// A method which tidies up long chains of dot operators for example in match statements.
    ///
    /// The `dud()` function makes an expression return () to avoid doing so with curly braces and a semicolon.
    ///
    /// # Example
    /// ```rust
    /// use crate::logicx::Dud;
    ///
    /// let mut v = vec![1, 2, 3];
    ///
    /// fn rand() -> f64 {
    ///     0.3
    /// }
    ///
    /// match rand() {
    ///  x if x <= 0.5 => v.push(10),
    ///  _ => v.get(0).dud(), // Do something which happens to return something.
    /// // equivalent
    ///  _ => {
    ///     v.get(0);
    ///  }
    /// }
    #[inline]
    fn dud(self) where Self: Sized {}
}

impl<T> Dud for T where T: Sized {}