use crate::project::Placement;
use crate::project::Project;
use crate::{State, Terminal, WireConnectStart};
use leptos::prelude::*;
use leptos::web_sys::SvgElement;

#[component]
pub fn logicx_component(placement: Placement) -> impl IntoView {
    let project = use_context::<RwSignal<Project>>().expect("Failed to get project");

    let state = use_context::<RwSignal<State>>().expect("Failed to get state");

    let inputs = move || {
        project.with(|project| {
            project
                .components
                .get(&placement.component)
                .map(|i| i.inputs.len())
                .unwrap_or(0)
        })
    };
    let outputs = move || {
        project.with(|project| {
            project
                .components
                .get(&placement.component)
                .map(|i| i.outputs.len())
                .unwrap_or(0)
        })
    };

    view!(<svg class="logicx-component" x={move || placement.pos.0 * state.read().grid_scale} y={move || placement.pos.1 * state.read().grid_scale}>
        <rect class="logicx-component-outline" rx=5
              width={move || inputs().min(outputs()).max(1) as f64 * state.read().grid_scale}
              height={move || inputs().max(outputs()).max(1) as f64 * state.read().grid_scale} />

        // {placement.label.map(|label| view!(<text x=0 y=0>{label}</text>))}

        <g class="logicx-input-terminals">{(0..inputs())
            .map(|i| view!(<circle class="logicx-component-terminal"
                r=5
                cx=0
                cy={i as f64 * state.read().grid_scale + state.read().grid_scale / 2.0}
                on:mousedown=move |e| if e.button() == 0 {
                    state.update(|state| state.start_connect_wire = Some(WireConnectStart {
                        from: placement.instance,
                        start_terminal: Terminal::Input(i as u64),
                        to: if let Some(bound) = state.viewport.with(|el| el.as_ref().map(|i: &SvgElement| i.get_bounding_client_rect())) {
                            (e.x() as f64 - bound.x() as f64, e.y() as f64 - bound.y() as f64)
                        } else {
                            (e.x() as f64, e.y() as f64)
                        }
                    }))
                } />))
            .collect_view()}</g>
        <g class="logicx-output-terminals">{(0..outputs())
            .map(|i| view!(<circle class="logicx-component-terminal"
                r=5
                cx=move || inputs().min(outputs()).max(1) as f64 * state.read().grid_scale
                cy={i as f64 * state.read().grid_scale + state.read().grid_scale / 2.0}
                on:mousedown=move |e| if e.button() == 0 {
                    state.update(|state| state.start_connect_wire = Some(WireConnectStart {
                        from: placement.instance,
                        start_terminal: Terminal::Output(i as u64),
                        to: if let Some(bound) = state.viewport.with(|el| el.as_ref().map(|i: &SvgElement| i.get_bounding_client_rect())) {
                            (e.x() as f64 - bound.x() as f64, e.y() as f64 - bound.y() as f64)
                        } else {
                            (e.x() as f64, e.y() as f64)
                        }
                    }))
                }/>))
            .collect_view()}</g>
    </svg>)
}
