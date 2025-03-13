use crate::{
    State,
    project::Connection,
    project::DragHandler,
    project::InstanceId,
    project::MouseState,
    project::Project,
    project::Terminal,
    project::Wire,
    WireConnectStart
};
use signal::signal;
use leptos::prelude::*;
use web_sys::MouseEvent;

fn component_mouse_down(e: MouseEvent, instance: InstanceId) -> Option<()> {
    let mouse = use_context::<RwSignal<DragHandler>>()?;
    let project = use_context::<ArcRwSignal<Project>>()?;
    let state = use_context::<ArcRwSignal<State>>()?;


    // if e.button() == 0 && mouse.read().is_some() {
    //
    //     let mut mouse = mouse.write();
    //     mouse = Some();
    //
    //     mouse.update(move |mouse| {
    //         if mouse.is_none() {
    //             mouse.replace(
    //                 MouseState::begin(e)
    //                     .start_coord(
    //                         project
    //                             .read()
    //                             .body
    //                             .get(&instance)
    //                             .map(|placement| placement.pos)
    //                             .unwrap_or_default(),
    //                     )
    //                     .on_move(move |mouse| {
    //                         use_context::<ArcRwSignal<Project>>()
    //                             .expect("Failed to get project")
    //                             .update(move |project| {
    //                                 if let Some(placement) = project.body.get_mut(&instance) {
    //                                     state.with(|state| {
    //                                         placement.pos = mouse.start_coord.unwrap_or_default()
    //                                             + (mouse.delta() / state.grid_scale);
    //
    //                                         if state.snap {
    //                                             placement.pos = placement.pos.quant(0.25);
    //                                         }
    //                                     });
    //                                 }
    //                             });
    //                     }),
    //             );
    //         }
    //     });
    // }

    let project = &project;
    let on_move = move |mouse| {
        let mut project = project.clone().write();
        // project.body.get_mut();
    };

    let project = project.clone();
    Some(signal!(|mut project, mut mouse, state| {
        if e.button() != 0 || mouse.is_some() {
            return None;
        }

        let state = MouseState::begin(e)
            .start_coord(project.body.get(&instance)?.pos)
            .on_move(on_move);
    }))
}

fn terminal_mouse_down(e: MouseEvent, terminal: Terminal) {
    // if e.button() == 0 {
    //     e.prevent_default();
    //
    //     mouse.update(move |mouse| {
    //         if mouse.is_none() {
    //             mouse.replace(
    //                 MouseState::begin(e)
    //                     .on_move(move |mouse| {
    //                         let state = use_context::<ArcRwSignal<State>>()
    //                             .expect("Failed to get state")
    //                             .clone();
    //
    //                         state.update(move |state| {
    //                             state.start_connect_wire.replace(WireConnectStart {
    //                                 from: instance,
    //                                 start_terminal: terminal,
    //                                 to: mouse.current_pos - state.viewport(),
    //                             });
    //                         });
    //                     })
    //                     .on_release(move |mouse| {
    //                         // Drop wire
    //                         let state = use_context::<ArcRwSignal<State>>()
    //                             .expect("Failed to get state")
    //                             .clone();
    //
    //                         state.update(|state| {
    //                             state.start_connect_wire.take();
    //                         })
    //                     }),
    //             );
    //         }
    //     });
    // }
}

fn terminal_mouse_up(e: MouseEvent, terminal: Terminal) {
    // mouse.update(move |mouse| {
    //     if let Some(mouse) = mouse.take() {
    //         if mouse.button == e.button() {
    //             let state = use_context::<ArcRwSignal<State>>()
    //                 .expect("Failed to get state")
    //                 .clone();
    //
    //             let project = use_context::<ArcRwSignal<Project>>().expect("Failed to get project");
    //
    //             state.update(|state| {
    //                 if let Some(start) = state.start_connect_wire.take() {
    //                     let (input, output) =
    //                         match ((start.from, start.start_terminal), (instance, terminal)) {
    //                             (
    //                                 (input_instance, Terminal::Input(i)),
    //                                 (output_instance, Terminal::Output(o)),
    //                             ) => (
    //                                 Connection::input(input_instance, i),
    //                                 Connection::output(output_instance, o),
    //                             ),
    //                             (
    //                                 (output_instance, Terminal::Output(o)),
    //                                 (input_instance, Terminal::Input(i)),
    //                             ) => (
    //                                 Connection::input(input_instance, i),
    //                                 Connection::output(output_instance, o),
    //                             ),
    //                             _ => return,
    //                         };
    //
    //                     project.update(|project| {
    //                         if let Some(con) = project.connections.get_mut(&output) {
    //                             con.push(input);
    //                         } else {
    //                             project.connections.insert(output, vec![input]);
    //                         }
    //
    //                         project.wires.push(Wire {
    //                             from: output.instance,
    //                             from_terminal: output.terminal,
    //                             points: vec![],
    //                             to: input.instance,
    //                             to_terminal: input.terminal,
    //                         });
    //                     });
    //                 }
    //             })
    //         }
    //     }
    // });
}

#[component]
pub fn logicx_component(instance: InstanceId) -> impl IntoView {
    let mouse = use_context::<DragHandler>().expect("Failed to get state");

    let size = Signal::derive(move || {
        let project = use_context::<ArcRwSignal<Project>>()
            .expect("Failed to get project")
            .read();

        let component = project
            .body
            .get(&instance)
            .and_then(|placement| project.components.get(&placement.component));

        component
            .map(|comp| (comp.inputs.len(), comp.outputs.len()))
            .unwrap_or_default()
    });

    let grid_scale = Signal::derive(move || {
        use_context::<ArcRwSignal<State>>()
            .expect("Failed to get state")
            .clone()
            .read()
            .grid_scale
    });

    let project = &use_context::<ArcRwSignal<Project>>().expect("Failed to get project");
    signal!(|grid_scale, size, project| {
        let grid_scale = *grid_scale;
        let size = size.clone();
        let pos = project.body.get(&instance).map(|placement| placement.pos).unwrap_or_default();

        view!(<svg class="logicx-component"
            x=move || pos.0 * grid_scale
            y=move || pos.1 * grid_scale
            on:mousedown=move |e| component_mouse_down(e, instance).unwrap_or(())>

            <rect class="logicx-component-outline" rx=5
              width=move || size.0.min(size.1).max(1) as f64 * grid_scale
              height=move || size.0.max(size.1).max(1) as f64 * grid_scale />

            // {placement.label.map(|label| view!(<text x=0 y=0>{label}</text>))}

            <g class="logicx-input-terminals">{(0..size.0)
                .map(|i| (i as u64, Terminal::Input(i as u64)))
                .map(|(i, start_terminal)| view!(<circle class="logicx-component-terminal"
                    r=5
                    cx=0
                    cy=move || i as f64 * grid_scale + grid_scale / 2.0
                    on:mousedown=move |e| terminal_mouse_down(e, start_terminal)
                    on:mouseup=move |e| terminal_mouse_up(e, start_terminal)
                    />))
                .collect_view()}</g>
            <g class="logicx-output-terminals">{(0..size.1)
                .map(|i| (i as u64, Terminal::Output(i as u64)))
                .map(|(i, start_terminal)| view!(<circle class="logicx-component-terminal"
                    r=5
                    cx=move || size.0.min(size.1).max(1) as f64 * grid_scale
                    cy=move || i as f64 * grid_scale + grid_scale / 2.0
                    on:mousedown=move |e| terminal_mouse_down(e, start_terminal)
                    on:mouseup=move |e| terminal_mouse_up(e, start_terminal)
                    />))
                .collect_view()}</g>
        </svg>)
    })
}
