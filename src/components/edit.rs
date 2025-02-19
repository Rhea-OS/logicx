use leptos::prelude::*;
use crate::{
    project::Terminal,
    project::Project,
    components::surface::LogicxSurface,
    components::component::LogicxComponent,
    wire::LogicxWire,
    State
};

#[component]
pub fn edit_mode() -> impl IntoView {

    let project = use_context::<RwSignal<Project>>().expect("Failed to get project");
    let state = use_context::<RwSignal<State>>().expect("Failed to get state");

    view!(<LogicxSurface>
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
    </LogicxSurface>)
}