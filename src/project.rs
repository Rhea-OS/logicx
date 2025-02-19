use leptos::prelude::*;
use regex::Regex;
use serde::{
    de::Unexpected,
    de::Visitor,
    Deserialize,
    Deserializer,
    Serialize,
    Serializer
};
use std::{
    cell::LazyCell,
    collections::HashMap,
    fmt::Display,
    fmt::Formatter,
    hash::Hash,
    ops::Add,
    ops::AddAssign,
    ops::Div,
    ops::Mul,
    ops::Sub,
    ops::SubAssign
};
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
    pub(crate) prev_pos: Coord,

    pub button: i16,

    pub start_coord: Option<Coord>, // Used to capture the starting coordinates of the object being dragged

    pub(crate) on_move: Option<Box<dyn Fn(&Self) + Send + Sync>>,
    pub(crate) on_release: Option<Box<dyn FnOnce(Self) + Send + Sync>>,
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