use std::cell::{LazyCell, OnceCell};
use crate::component::LogicxComponent;
use crate::wire::LogicxWire;
use crate::{ContextProvider, State};
use leptos::prelude::*;
use serde::{Deserialize, Deserializer, Serializer};
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};
use regex::Regex;
use serde::de::{Unexpected, Visitor};
use web_sys::MouseEvent;

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub(crate) components: HashMap<ComponentId, Component>,

    pub(crate) body: HashMap<InstanceId, Placement>,

    // The datastructure holds connections in a logically-reversed order to facilitate 1-n relationship
    // Connections are represented as _Output feeds the following inputs_
    pub(crate) connections: HashMap<Connection, Vec<Connection>>,

    // TODO: convert (InstanceId, u64) into a string-serialisable type

    pub(crate) wires: Vec<Wire>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Connection {
    pub(crate) instance: InstanceId,
    pub(crate) terminal: Terminal
}

impl Connection {
    pub(crate) fn input(instance: InstanceId, terminal: u64) -> Self {
        Self {
            instance,
            terminal: Terminal::Input(terminal)
        }
    }

    pub(crate) fn output(instance: InstanceId, terminal: u64) -> Self {
        Self {
            instance,
            terminal: Terminal::Output(terminal)
        }
    }
}

impl Display for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self { instance, terminal: Terminal::Input(i) } => write!(f, "I{}:{}", instance, i),
            Self { instance, terminal: Terminal::Output(i) } => write!(f, "O{}:{}", instance, i),
        }
    }
}

impl Serialize for Connection {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Connection {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_str(ConnectionVisitor)
    }
}

struct  ConnectionVisitor;
const CONNECTION_REGEX: LazyCell<Regex> = LazyCell::new(|| Regex::new(r"^(?<type>[IOio])(?<instance>\d+):(?<terminal>\d+)$").expect("Failed to parse RegExp"));

impl Visitor<'_> for ConnectionVisitor {
    type Value = Connection;

    fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Expecting a connection of format /^[IOio]instance:terminal$/")
    }

    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let connection = CONNECTION_REGEX.captures(v)
            .ok_or(serde::de::Error::custom("Connection does not match the expected format"))?;

        let instance = connection.name("instance")
            .ok_or(serde::de::Error::missing_field("instance"))
            .and_then(|i| i.as_str().parse().map_err(|err| {
                serde::de::Error::invalid_type(Unexpected::Str(i.as_str()), &"Instance")
            }))?;

        let terminal = connection.name("terminal")
            .ok_or(serde::de::Error::missing_field("terminal"))
            .and_then(|i| i.as_str().parse().map_err(|err| {
                serde::de::Error::invalid_type(Unexpected::Str(i.as_str()), &"Terminal")
            }))?;

        let r#type = match connection.name("type").map(|i| i.as_str()) {
            Some("i") | Some("I") => Ok(Terminal::Input(terminal)),
            Some("o") | Some("O") => Ok(Terminal::Output(terminal)),
            Some(i) => Err(serde::de::Error::invalid_value(Unexpected::Str(i), &"Terminal type must be one of 'ioIO'")),
            None => Err(serde::de::Error::missing_field("terminal")),
        }?;

        Ok(Connection {
            instance,
            terminal: r#type
        })
    }
}

impl Hash for Connection {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
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
                        pos: (0.0, 0.0).into(),
                        orientation: 0.0,
                    },
                ),
                (
                    1,
                    Placement {
                        component: 4,
                        instance: 1,

                        label: Some("Output".to_string()),
                        pos: (0.0, 1.0).into(),
                        orientation: 0.0,
                    },
                ),
                (
                    2,
                    Placement {
                        component: 2,
                        instance: 2,

                        label: Some("And".to_string()),
                        pos: (2.0, 0.0).into(),
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

    pub(crate) pos: Coord,
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
    pub fn truth(truth: impl IntoIterator<Item=(u64, u64)>) -> Self {
        Self::TruthTable {
            truth: truth.into_iter().collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Script {
    pub(crate) script: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Wire {
    pub(crate) from: InstanceId,
    pub(crate) from_terminal: Terminal,

    pub(crate) points: Vec<Coord>,

    pub(crate) to: InstanceId,
    pub(crate) to_terminal: Terminal,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Terminal {
    Input(u64),
    Output(u64),
}

pub struct MouseState {
    pub start_pos: Coord,
    pub current_pos: Coord,
    prev_pos: Coord,

    pub button: i16,

    pub start_coord: Option<Coord>, // Used to capture the starting coordinates of the object being dragged

    on_move: Option<Box<dyn Fn(&Self) + Send + Sync>>,
    on_release: Option<Box<dyn FnOnce(Self) + Send + Sync>>,
}

impl MouseState {
    pub fn begin(e: MouseEvent) -> Self {
        Self {
            start_pos: (e.x() as f64, e.y() as f64).into(),
            current_pos: (e.x() as f64, e.y() as f64).into(),
            prev_pos: (e.x() as f64, e.y() as f64).into(),

            start_coord: None,

            on_move: None,
            on_release: None,

            button: e.button(),
        }
    }

    pub fn start_coord(mut self, start: Coord) -> Self {
        self.start_coord = Some(start);
        self
    }

    pub fn on_move<T: Fn(&Self) + Send + Sync + 'static>(mut self, f: T) -> Self {
        self.on_move = Some(Box::new(f));
        self
    }

    pub fn on_release<T: FnOnce(Self) + Send + Sync + 'static>(mut self, f: T) -> Self {
        self.on_release = Some(Box::new(f));
        self
    }

    pub fn delta(&self) -> Coord {
        (self.current_pos.0 - self.start_pos.0, self.current_pos.1 - self.start_pos.1).into()
    }

    pub fn delta_inv(&self) -> Coord {
        (self.start_pos.0 - self.current_pos.0, self.start_pos.1 - self.current_pos.1).into()
    }

    pub fn delta_tick(&self) -> Coord {
        (self.current_pos.0 - self.prev_pos.0, self.current_pos.1 - self.prev_pos.1).into()
    }

    pub fn delta_tick_inv(&self) -> Coord {
        (self.prev_pos.0 - self.current_pos.0, self.prev_pos.1 - self.current_pos.1).into()
    }
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Coord(pub f64, pub f64);

impl AddAssign for Coord {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

impl SubAssign for Coord {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.1 -= other.1;
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Div<f64> for Coord {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

impl Mul<f64> for Coord {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl From<(f64, f64)> for Coord {
    fn from(value: (f64, f64)) -> Self {
        Self(value.0, value.1)
    }
}

impl Coord {
    pub fn quant(self, scale: f64) -> Self {
        Self(self.0 - self.0 % scale, self.1 - self.1 % scale)
    }
}

pub type DragHandler = RwSignal<Option<MouseState>>;

#[component]
pub fn project() -> impl IntoView {
    let selection = RwSignal::<Vec<InstanceId>>::new(vec![]);

    let project = use_context::<RwSignal<Project>>().expect("Failed to get project");
    let state = use_context::<RwSignal<State>>().expect("Failed to get state");

    let mouse = DragHandler::new(None);

    view!(<ContextProvider cx=mouse>
        <svg class="logicx-surface" xmlns="http://www.w3.org/2000/svg"
            on:wheel=move |e| state.update(|state| if e.shift_key() {
                state.scroll -= (e.delta_y(), e.delta_x()).into()
            } else {
                state.scroll -= (e.delta_x(), e.delta_y()).into()
            })

            on:mousedown=move |e| mouse.update(|mouse| {
                if e.button() == 1 {
                    mouse.replace(MouseState::begin(e)
                        .on_move(move |mouse| state.update(move |state| {
                            state.scroll += mouse.delta_tick();
                        })));
                }
            })

            on:mousemove=move |e| mouse.update(|mouse| if let Some(mouse) = mouse.as_mut() {
                mouse.prev_pos = mouse.current_pos;
                mouse.current_pos = Coord(e.x() as f64, e.y() as f64);

                if let Some(ref onmove) = mouse.on_move {
                    onmove(mouse)
                }
            })
            on:mouseup=move |e| mouse.update(|mouse| match mouse.take() {
                Some(mut mouse) if mouse.button == e.button() => if let Some(on_release) = mouse.on_release.take() {
                    on_release(mouse)
                },
                _ => {}
            })>

            <defs>
                <pattern id="grid-small"
                    x=0 y=0
                    width=move || state.get().grid_scale / 4.0
                    height=move || state.get().grid_scale / 4.0
                    patternUnits="userSpaceOnUse">

                    // <path d="M 0 0 L 0 0" stroke="grey" fill="none" stroke-opacity="0.25" stroke-width="0.25" width=move || state.get().grid_scale / 4.0 height=move || state.get().grid_scale / 4.0 />
                    // <rect x=0 y=0 width=move || state.get().grid_scale / 4.0 height=move || state.get().grid_scale / 4.0 stroke="grey" fill="none" stroke-opacity="0.25" stroke-width="0.25" />

                    <circle r=1 cx=0 cy=0 fill="grey" fill-opacity="0.25" />
                </pattern>
                <pattern id="grid"
                    x=move || state.get().scroll.0
                    y=move || state.get().scroll.1
                    width=move || state.get().grid_scale
                    height=move || state.get().grid_scale
                    patternUnits="userSpaceOnUse">

                    <rect x=0 y=0 width=move || state.get().grid_scale height=move || state.get().grid_scale fill="url(#grid-small)" />

                    <circle r=1 cx=0 cy=0 fill="grey" fill-opacity="0.5" />
                </pattern>
            </defs>

            <Show when=move || state.get().snap>
                <rect fill="url(#grid)" width="100%" height="100%" />
            </Show>

            <svg node_ref=state.with(|state| state.viewport)
                x=move || state.with(|state| state.scroll.0)
                y=move || state.with(|state| state.scroll.1)>

                <g class="wires">
                    {move || project.with(|project| project.wires.iter()
                        .map(|wire| view!(<LogicxWire wire=wire.clone() />))
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
                <g class="components">
                    {move || project
                        .with(|project| project
                            .body.iter()
                            .map(|(instance, placement)| view!(<LogicxComponent instance=placement.instance />))
                            .collect_view())}
                </g>
            </svg>
        </svg>
    </ContextProvider>)
}
