pub mod error;
pub mod components;
pub mod project;

use crate::components::*;
use std::rc::Rc;
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
    state: RwSignal<State>,
}

#[wasm_bindgen(js_class=LogicXContext)]
impl LogicX {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            project: RwSignal::new(Project::empty()),
            state: RwSignal::new(State::new()),
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

                <Show when=move || state.try_read().map(|state| state.edit).unwrap_or_default()
                    fallback=move || view!(<PlayMode />)>
                    <EditMode />
                </Show>
            </ContextProvider>
        </ContextProvider>)).forget();
    }


    #[wasm_bindgen(js_name=getData)]
    pub fn get_data(&self) -> String {
        match serde_json::to_string_pretty(&self.project) {
            Ok(project) => project,
            Err(err) => panic!("Panic: {:?}", err)
        }
    }

    #[wasm_bindgen(js_name=setData)]
    pub fn set_data(&mut self, data: String, clear: bool) {
        match serde_json::from_str(data.as_str()) {
            Ok(project) => self.project.set(project),
            Err(err) => panic!("Panic: {:?}", err)
        }
    }

    #[wasm_bindgen(js_name=clear)]
    pub fn clear(&mut self) {
        self.project.set(Project::empty());
    }

    #[wasm_bindgen(js_name=getState)]
    pub fn get_state(&self) -> State {
        self.state.read_untracked().clone()
    }

    #[wasm_bindgen(js_name=setState)]
    pub fn set_state(&self, new_state: &State) {
        self.state.update(|state| *state = new_state.clone());
    }

    #[wasm_bindgen(js_name=onStateChanged, typescript_type = "(state: State) => void")]
    pub fn on_state_changed(&self, listener: js_sys::Function) {
        let state = self.state.clone();
        Effect::new(move |_| listener.call1(&JsValue::null(), &JsValue::from(state.get())));
    }
}

#[derive(Clone)]
pub(crate) struct WireConnectStart {
    pub(crate) from: InstanceId,
    pub(crate) start_terminal: Terminal,
    pub(crate) to: Coord
}

#[derive(Clone, Default)]
#[wasm_bindgen(js_name=LogicXState)]
pub struct State {
    pub(crate) scroll: Coord,
    pub(crate) grid_scale: f64,
    pub(crate) snap: bool,

    pub(crate) start_connect_wire: Option<WireConnectStart>,

    pub(crate) viewport: NodeRef<Svg>,

    pub(crate) edit: bool
}

#[wasm_bindgen(js_class=LogicXState)]
impl State {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        State {
            grid_scale: 35.0,
            viewport: NodeRef::new(),
            snap: true,
            scroll: (0.0, 0.0).into(),
            start_connect_wire: None,
            edit: true
        }
    }

    #[wasm_bindgen(getter)]
    pub fn grid_scale(&self) -> f64 {
        self.grid_scale
    }

    #[wasm_bindgen(getter)]
    pub fn scroll(&self) -> WasmCoord {
        (self.scroll.0, self.scroll.1).into()
    }

    #[wasm_bindgen(getter)]
    pub fn edit(&self) -> bool {
        self.edit
    }

    #[wasm_bindgen(getter)]
    pub fn snap(&self) -> bool {
        self.snap
    }


    #[wasm_bindgen(setter)]
    pub fn set_grid_scale(&mut self, value: f64) {
        self.grid_scale = value;
    }

    #[wasm_bindgen(setter)]
    pub fn set_scroll(&mut self, value: WasmCoord) {
        self.scroll.0 = value.x;
        self.scroll.1 = value.y;
    }

    #[wasm_bindgen(setter)]
    pub fn set_edit(&mut self, value: bool) {
        self.edit = value;
    }

    #[wasm_bindgen(setter)]
    pub fn set_snap(&mut self, value: bool) {
        self.snap = value;
    }

    #[wasm_bindgen(js_name=withGridScale)]
    pub fn with_grid_scale(mut self, value: f64) -> Self {
        self.grid_scale = value;
        return self;
    }

    #[wasm_bindgen(js_name=withScroll)]
    pub fn with_scroll(mut self, value: WasmCoord) -> Self {
        self.scroll.0 = value.x;
        self.scroll.1 = value.y;
        return self;
    }

    #[wasm_bindgen(js_name=withEdit)]
    pub fn with_edit(mut self, value: bool) -> Self {
        self.edit = value;
        return self;
    }

    #[wasm_bindgen(js_name=withSnap)]
    pub fn with_snap(mut self, value: bool) -> Self {
        self.snap = value;
        return self;
    }
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

#[wasm_bindgen(js_name=LogicXCoord)]
pub struct WasmCoord {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen(js_class=LogicXCoord)]
impl WasmCoord {
    #[wasm_bindgen(constructor)]
    pub fn x(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl Into<Coord> for WasmCoord {
    fn into(self) -> Coord {
        Coord(self.x, self.y)
    }
}

impl From<(f64, f64)> for WasmCoord {
    fn from((x, y): (f64, f64)) -> Self {
        Self { x, y }
    }
}

impl From<Coord> for WasmCoord {
    fn from(coord: Coord) -> Self {
        WasmCoord { x: coord.0, y: coord.1 }
    }
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