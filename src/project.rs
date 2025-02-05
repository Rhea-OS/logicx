use crate::component::LogicxComponent;
use crate::wire::LogicxWire;
use crate::{LogicX, State};
use leptos::logging;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use web_sys::SvgElement;

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub(crate) components: HashMap<ComponentId, Component>,

    pub(crate) body: HashMap<InstanceId, Placement>,
    pub(crate) connections: HashMap<(ComponentId, String), (ComponentId, String)>,

    pub(crate) wires: Vec<Wire>,
}

impl Project {
    pub fn empty() -> Self {
        Self {
            components: vec![
                (
                    0,
                    Component {
                        id: 0,

                        name: "not".into(),

                        inputs: vec!["q".into()],
                        outputs: vec!["q!".into()],

                        driver: ComponentDriver::truth([(0b0, 0b1), (0b1, 0b0)]),
                    },
                ),
                (
                    1,
                    Component {
                        id: 1,

                        name: "and".into(),

                        inputs: vec!["a".into(), "b".into()],
                        outputs: vec!["and".into()],

                        driver: ComponentDriver::truth([
                            (0b00, 0b0),
                            (0b01, 0b0),
                            (0b10, 0b0),
                            (0b11, 0b1),
                        ]),
                    },
                ),
                (
                    2,
                    Component {
                        id: 2,

                        name: "or".into(),

                        inputs: vec!["a".into(), "b".into()],
                        outputs: vec!["or".into()],

                        driver: ComponentDriver::truth([
                            (0b00, 0b0),
                            (0b01, 0b1),
                            (0b10, 0b1),
                            (0b11, 0b1),
                        ]),
                    },
                ),
                (
                    3,
                    Component {
                        id: 3,

                        name: "input".into(),

                        inputs: vec![],
                        outputs: vec!["q".into()],

                        driver: ComponentDriver::Input,
                    },
                ),
                (
                    4,
                    Component {
                        id: 4,

                        name: "output".into(),

                        inputs: vec!["q".into()],
                        outputs: vec![],

                        driver: ComponentDriver::Output,
                    },
                ),
            ]
            .into_iter()
            .collect(),

            body: vec![
                (
                    0,
                    Placement {
                        component: 3,
                        instance: 0,

                        label: Some("Input".to_string()),
                        pos: (0.0, 0.0),
                        orientation: 0.0,
                    },
                ),
                (
                    1,
                    Placement {
                        component: 4,
                        instance: 1,

                        label: Some("Output".to_string()),
                        pos: (0.0, 1.0),
                        orientation: 0.0,
                    },
                ),
                (
                    2,
                    Placement {
                        component: 2,
                        instance: 2,

                        label: Some("And".to_string()),
                        pos: (2.0, 0.0),
                        orientation: 0.0,
                    },
                ),
            ]
            .into_iter()
            .collect(),
            connections: HashMap::new(),
            wires: vec![],
        }
    }
}

pub type ComponentId = usize;
pub type InstanceId = usize;

#[derive(Serialize, Deserialize)]
pub struct Component {
    pub(crate) id: ComponentId,

    pub(crate) name: String,

    pub(crate) inputs: Vec<String>,
    pub(crate) outputs: Vec<String>,

    pub(crate) driver: ComponentDriver,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Placement {
    pub(crate) component: ComponentId,
    pub(crate) instance: InstanceId,

    pub(crate) label: Option<String>,

    pub(crate) pos: (f64, f64),
    pub(crate) orientation: f64,
}

#[derive(Serialize, Deserialize)]
pub enum ComponentDriver {
    TruthTable {
        truth: HashMap<u64, u64>,
    },
    Subcomponent {
        connections: HashMap<(ComponentId, String), (ComponentId, String)>,
    },
    Script {
        script: Script,
    },

    Input,
    Output,
}

impl ComponentDriver {
    pub fn truth(truth: impl IntoIterator<Item = (u64, u64)>) -> Self {
        Self::TruthTable {
            truth: truth.into_iter().collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Script {
    pub(crate) script: String,
}

#[derive(Serialize, Deserialize)]
pub struct Wire {
    from: InstanceId,
    from_terminal: Terminal,

    points: Vec<(f64, f64)>,

    to: InstanceId,
    to_terminal: Terminal,
}

#[derive(Serialize, Deserialize)]
pub enum Terminal {
    Input(u64),
    Output(u64),
}

#[component]
pub fn project() -> impl IntoView {
    let selection = RwSignal::<Vec<InstanceId>>::new(vec![]);

    let project = use_context::<RwSignal<Project>>().expect("Failed to get project");
    let state = use_context::<RwSignal<State>>().expect("Failed to get state");

    struct Scroll {
        mouse_x: f64,
        mouse_y: f64,
        start_x: f64,
        start_y: f64,
    }

    let scroll = RwSignal::<Option<Scroll>>::new(None);

    view!(<svg class="logicx-surface" xmlns="http://www.w3.org/2000/svg"
        on:wheel=move |e| state.update(|state| if e.shift_key() {
            state.scroll = (state.scroll.0 - e.delta_y(), state.scroll.1 - e.delta_x())
        } else {
            state.scroll = (state.scroll.0 - e.delta_x(), state.scroll.1 - e.delta_y())
        })
        on:mousedown=move |e| if e.button() == 1 {
            let current_scroll = state.with(|state| state.scroll);

            scroll.set(Some(Scroll {
                mouse_x: e.x() as f64,
                mouse_y: e.y() as f64,
                start_x: current_scroll.0,
                start_y: current_scroll.1
            }));
        }
        on:mousemove=move |e| {
            scroll.with(|scroll| if let Some(scroll) = scroll {
                state.update(|state| state.scroll = (
                    scroll.start_x + (e.x() as f64 - scroll.mouse_x),
                    scroll.start_y + (e.y() as f64 - scroll.mouse_y),
                ))
            });

            state.update(|state| if let Some(wire) = state.start_connect_wire.as_mut() {
                wire.to = if let Some(bound) = state.viewport.with(|el| el.as_ref().map(|i: &SvgElement| i.get_bounding_client_rect())) {
                    (e.x() as f64 - bound.x() as f64, e.y() as f64 - bound.y() as f64)
                } else {
                    (e.x() as f64, e.y() as f64)
                }
            });
        }
        on:mouseup=move |e| if e.button() == 1 {
            scroll.set(None);
        } else if e.button() == 0 {
            state.update(|state| state.start_connect_wire = None)
        }>
        <svg node_ref=state.with(|state| state.viewport)
        x=move || state.with(|state| state.scroll.0)
        y=move || state.with(|state| state.scroll.1)>
            <g class="components">
                {move || project
                    .with(|project| project
                        .body.iter()
                        .map(|(instance, placement)| view!(<LogicxComponent placement=placement.clone() />))
                        .collect_view())}
            </g>
            <g class="wires">
                {move || project.with(|project| project.wires.iter()
                    .map(|wire| view!(<LogicxWire />))
                    .collect_view())}

                {move || state.with(|state| state.start_connect_wire.as_ref().map(|wire| {
                    let from = project.with(|project| project.body.get(&wire.from).cloned())?;
                    let (inputs, outputs) = project.with(|project| project.components.get(&from.component).map(|comp| (
                        comp.inputs.len(),
                        comp.outputs.len()
                    )))?;

                    let (dx, dy) = match wire.start_terminal {
                        Terminal::Input(terminal) => (0.0, terminal as f64 * state.grid_scale + state.grid_scale / 2.0),
                        Terminal::Output(terminal) => (inputs.min(outputs).max(1) as f64 * state.grid_scale, terminal as f64 * state.grid_scale + state.grid_scale / 2.0)
                    };

                    Some(view!(<path class="logicx-wire" d=format!("M {sx} {sy} C {mx} {sy}, {sx} {my}, {mx} {my}",
                        sx = from.pos.0 * state.grid_scale + dx,
                        sy = from.pos.1 * state.grid_scale + dy,
                        mx = wire.to.0,
                        my = wire.to.1) />))
                }))}
            </g>
        </svg>
    </svg>)
}
