use std::ops::Deref;
use leptos::prelude::*;
use leptos::logging;
use crate::project::{Coord, Project, Terminal, Wire};
use crate::State;

#[component]
pub fn logicx_wire(wire: Wire) -> impl IntoView {
    let project = use_context::<RwSignal<Project>>()?;
    let from = Signal::derive(move || project.read().body.get(&wire.from).cloned());
    let size = Signal::derive(move || from.read().as_ref()
        .map(|i| i.component)
        .and_then(|from| project.read().components
            .get(&from)
            .map(|comp| (
                comp.inputs.len(),
                comp.outputs.len()
            ))));

    let state = use_context::<RwSignal<State>>()?;
    let delta = Signal::derive(move || {
        let state = state.read();
        let (inputs, outputs) = size.read().unwrap_or_default();

        match wire.from_terminal {
            Terminal::Input(terminal) => (0.0, terminal as f64 * state.grid_scale + state.grid_scale / 2.0),
            Terminal::Output(terminal) => (inputs.min(outputs).max(1) as f64 * state.grid_scale, terminal as f64 * state.grid_scale + state.grid_scale / 2.0)
        }
    });

    let to = Signal::derive(move || {
        let state = state.read();
        let project = project.read();

        project.body.get(&wire.to)
            .map(|placement| placement.pos)
            .map(|Coord(x, y)| (
                x * state.grid_scale,
                y * state.grid_scale + state.grid_scale / 2.0 + match wire.to_terminal {
                    Terminal::Input(terminal) | Terminal::Output(terminal) => terminal as f64 * state.grid_scale,
                }
            ))
    });

    Some(move || -> Option<_> {
        let from = from.read().as_ref()?.clone();
        let to = to.read().as_ref()?.clone();

        let state = state.read();
        let delta = delta.read();

        Some(view!(<path class="logicx-wire" d=format!("M {sx} {sy} C {mx} {sy}, {sx} {my}, {mx} {my}",
            sx = from.pos.0 * state.grid_scale + delta.0,
            sy = from.pos.1 * state.grid_scale + delta.1,
            mx = to.0,
            my = to.1) />))
    })
}