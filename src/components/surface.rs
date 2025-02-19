use crate::project::{Coord, DragHandler, InstanceId, MouseState, Project};
use crate::{ContextProvider, State};
use leptos::prelude::*;
use web_sys::MouseEvent;

#[component]
pub fn logicx_surface(children: Children) -> impl IntoView {
    let selection = RwSignal::<Vec<InstanceId>>::new(vec![]);

    let project = use_context::<RwSignal<Project>>().expect("Failed to get project");
    let state = use_context::<RwSignal<State>>().expect("Failed to get state");

    let mouse = DragHandler::new(None);

    view!(<ContextProvider cx=mouse>
        <svg class="logicx-surface" class:play-mode=move || !state.read().edit xmlns="http://www.w3.org/2000/svg"
            on:wheel=move |e| state.update(|state| if e.shift_key() {
                state.scroll -= (e.delta_y(), e.delta_x()).into()
            } else {
                state.scroll -= (e.delta_x(), e.delta_y()).into()
            })

            on:mousedown=move |e: MouseEvent| mouse.update(|mouse| {
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

                    <circle r=1 cx=0 cy=0 class="backdrop-pattern" fill-opacity="0.25" />
                </pattern>
                <pattern id="grid"
                    x=move || state.get().scroll.0
                    y=move || state.get().scroll.1
                    width=move || state.get().grid_scale
                    height=move || state.get().grid_scale
                    patternUnits="userSpaceOnUse">

                    <rect x=0 y=0 width=move || state.get().grid_scale height=move || state.get().grid_scale fill="url(#grid-small)" />

                    <circle r=1 cx=0 cy=0 class="backdrop-pattern" fill-opacity="0.5" />
                </pattern>
            </defs>

            <Show when=move || state.get().snap>
                <rect fill="url(#grid)" width="100%" height="100%" />
            </Show>

            <svg node_ref=state.read_untracked().viewport
                x=move || state.with(|state| state.scroll.0)
                y=move || state.with(|state| state.scroll.1)>

                {children()}
            </svg>
        </svg>
    </ContextProvider>)
}
