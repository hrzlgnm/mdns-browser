use js_sys::{
    wasm_bindgen::{prelude::Closure, JsCast},
    Function,
};
use leptos::{ev::Event, prelude::*};
use thaw::Icon;

#[component]
pub fn BackTop(
    #[prop(optional, into)] threshold: Option<f64>, // show after N pixels scrolled by default
) -> impl IntoView {
    let threshold = threshold.unwrap_or(300.0);
    let visible = RwSignal::new(false);

    Effect::new(move |_| {
        let w = window();

        // initial visibility check
        visible.set(w.scroll_y().unwrap_or(0.0) > threshold);

        let scroll = Closure::<dyn FnMut(Event)>::wrap(Box::new(move |_| {
            let y = w.scroll_y().unwrap_or(0.0);
            visible.set(y > threshold);
        }));
        let w = window();
        if let Err(e) = w
            .add_event_listener_with_callback("scroll", scroll.as_ref().unchecked_ref::<Function>())
        {
            log::error!("Failed to add scroll event listener: {e:?}")
        }

        move || scroll.forget()
    });

    let on_click = move |_| {
        let w = window();
        w.scroll_to_with_x_and_y(0.0, 0.0);
    };

    view! {
        <div class="back-top-container">
            <Show when=move || visible.get()>
                <button class="back-top-button" on:click=on_click>
                    <Icon icon=icondata::MdiFormatVerticalAlignTop />
                </button>
            </Show>
        </div>
    }
}
