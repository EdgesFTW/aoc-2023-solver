use crate::components::input::*;
use leptos::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <div class="container">

                <picture>
                    <source
                        // srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg"
                        srcset="https://repository-images.githubusercontent.com/161316257/10d14a84-da93-4e38-b7cc-cd4c96c1eedc"
                        media="(prefers-color-scheme: dark)"
                    />
                    <img
                        // src="https://repository-images.githubusercontent.com/161316257/10d14a84-da93-4e38-b7cc-cd4c96c1eedc"
                        src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg"
                        alt="Leptos Logo"
                        height="200"
                        width="400"
                    />
                </picture>

                <h3>"Advent of Code 2023 Solver"</h3>

                <Aoc_input/>

            </div>
        </ErrorBoundary>
    }
}
