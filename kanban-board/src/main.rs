use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <div>
            <h1>"Hello, Leptos!"</h1>
            <p>"This is a minimal Kanban Board application."</p>
        </div>
    }
}

fn main() {
    leptos::mount::mount_to_body(App)
}
