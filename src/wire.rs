use leptos::prelude::*;
use leptos::logging;
use crate::project::{Coord, Project, Terminal, Wire};
use crate::State;

#[component]
pub fn logicx_wire(wire: Wire) -> impl IntoView {
    let project = use_context::<RwSignal<Project>>()?;
    let from = project.with(|project| project.body.get(&wire.from).cloned())?;
    let (inputs, outputs) = project.with(|project| project.components.get(&from.component).map(|comp| (
        comp.inputs.len(),
        comp.outputs.len()
    )))?;

    let (grid_scale, (dx, dy)) = use_context::<RwSignal<State>>()?
        .with(|state| (state.grid_scale, match wire.from_terminal {
            Terminal::Input(terminal) => (0.0, terminal as f64 * state.grid_scale + state.grid_scale / 2.0),
            Terminal::Output(terminal) => (inputs.min(outputs).max(1) as f64 * state.grid_scale, terminal as f64 * state.grid_scale + state.grid_scale / 2.0)
        }));

    let to = project
        .with(|project| project.body.get(&wire.to)
            .map(|i| i.pos)
            .map(|Coord(x, y)| (
                x * grid_scale,
                y * grid_scale + grid_scale / 2.0 + match wire.to_terminal {
                    Terminal::Input(terminal) | Terminal::Output(terminal) => terminal as f64 * grid_scale,
                }
            )))?;

    Some(view!(<path class="logicx-wire" d=format!("M {sx} {sy} C {mx} {sy}, {sx} {my}, {mx} {my}",
            sx = from.pos.0 * grid_scale + dx,
            sy = from.pos.1 * grid_scale + dy,
            mx = to.0,
            my = to.1) />))
}