use crate::component::LogicxComponent;
use crate::{LogicX, State};
use leptos::logging;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub(crate) components: HashMap<ComponentId, Component>,

    pub(crate) body: HashMap<InstanceId, Placement>,
    pub(crate) connections: HashMap<(ComponentId, String), (ComponentId, String)>,

    pub(crate) wires: Vec<Vec<(f64, f64)>>,
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

            body: vec![(0, Placement {
                component: 3,
                label: Some("Input".to_string()),
                pos: (0.0, 0.0),
                orientation: 0.0,
            }), (1, Placement {
                component: 4,
                label: Some("Output".to_string()),
                pos: (0.0, 1.0),
                orientation: 0.0,
            }), (2, Placement {
                component: 2,
                label: Some("And".to_string()),
                pos: (2.0, 0.0),
                orientation: 0.0,
            })]
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

#[component]
pub fn project() -> impl IntoView {
    let selection = RwSignal::<Vec<InstanceId>>::new(vec![]);

    let project = use_context::<RwSignal<Project>>().expect("Failed to get project");
    let state = use_context::<RwSignal<State>>().expect("Failed to get state");

    struct Scroll {
        mouse_x: f64, mouse_y: f64,
        start_x: f64, start_y: f64,
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
        on:mousemove=move |e| scroll.with(|scroll| if let Some(scroll) = scroll {
            state.update(|state| state.scroll = (
                scroll.start_x + (e.x() as f64 - scroll.mouse_x),
                scroll.start_y + (e.y() as f64 - scroll.mouse_y),
            ))
        })
        on:mouseup=move |e| if e.button() == 1 {
            scroll.set(None);
        }>
        <svg x=move || state.with(|state| state.scroll.0) y=move || state.with(|state| state.scroll.1)>
            <g class="components">
                {move || project
                    .with(|project| project
                        .body.iter()
                        .map(|(instance, placement)| view!(<LogicxComponent placement=placement.clone() />))
                        .collect_view())}
            </g>
            <g class="wires">

            </g>
        </svg>
    </svg>)
}
