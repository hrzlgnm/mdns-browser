use leptos::prelude::*;

#[component]
pub fn SafeBoundary(children: Children) -> impl IntoView {
    #[cfg(feature = "error_boundary")]
    {
        view! {
            <ErrorBoundary fallback=|errors| {
                view! {
                    <div class="error p-4 rounded bg-red-100 text-red-800">
                        <h2 class="font-bold text-lg">"Something went wrong"</h2>
                        <ul class="text-sm">
                            {move || {
                                errors
                                    .get()
                                    .into_iter()
                                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                    .collect::<Vec<_>>()
                            }}
                        </ul>
                    </div>
                }
            }>{children()}</ErrorBoundary>
        }
    }

    #[cfg(not(feature = "error_boundary"))]
    {
        view! { {children()} }
    }
}
