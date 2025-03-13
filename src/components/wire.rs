use crate::{project::Coord, project::Project, project::Terminal, project::Wire, State};
use leptos::prelude::*;
use signal::signal;

#[component]
pub fn logicx_wire(wire: Wire) -> impl IntoView {
    let from = Signal::derive(move || {
        use_context::<ArcRwSignal<Project>>()
            .and_then(|project| project.clone().read().body.get(&wire.from).cloned())
    });
    let size = Signal::derive(move || {
        use_context::<ArcRwSignal<Project>>().and_then(|project| {
            from.clone()
                .read()
                .as_ref()
                .map(|i| i.component)
                .and_then(|from| {
                    project
                        .clone()
                        .read()
                        .components
                        .get(&from)
                        .map(|comp| (comp.inputs.len(), comp.outputs.len()))
                })
        })
    });
    let delta = Signal::derive(move || {
        use_context::<ArcRwSignal<State>>().map(|state| {
            let state = state.clone().read();
            let (inputs, outputs) = size.read().unwrap_or_default();

            match wire.from_terminal {
                Terminal::Input(terminal) => (
                    0.0,
                    terminal as f64 * state.grid_scale + state.grid_scale / 2.0,
                ),
                Terminal::Output(terminal) => (
                    inputs.min(outputs).max(1) as f64 * state.grid_scale,
                    terminal as f64 * state.grid_scale + state.grid_scale / 2.0,
                ),
            }
        })
    });
    let to = Signal::derive(move || {
        use_context::<ArcRwSignal<State>>().and_then(|state| {
            let state = state.clone().read();
            use_context::<ArcRwSignal<Project>>().and_then(|project| {
                project
                    .read()
                    .body
                    .get(&wire.to)
                    .map(|placement| placement.pos)
                    .map(|Coord(x, y)| {
                        (
                            x * state.grid_scale,
                            y * state.grid_scale
                                + state.grid_scale / 2.0
                                + match wire.to_terminal {
                                    Terminal::Input(terminal) | Terminal::Output(terminal) => {
                                        terminal as f64 * state.grid_scale
                                    }
                                },
                        )
                    })
            })
        })
    });
    let state = use_context::<ArcRwSignal<State>>()?;

    signal!(|from, to, delta, state| {
        let from = from.as_ref()?.clone();
        let to = to.as_ref()?.clone();

        let delta = delta.as_ref()?.clone();

        Some(
            view!(<path class="logicx-wire" d=format!("M {sx} {sy} C {mx} {sy}, {sx} {my}, {mx} {my}",
            sx = from.pos.0 * state.grid_scale + delta.0,
            sy = from.pos.1 * state.grid_scale + delta.1,
            mx = to.0,
            my = to.1) />),
        )
    })
}
