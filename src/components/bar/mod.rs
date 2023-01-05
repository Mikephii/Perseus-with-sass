use sycamore::prelude::{component, view, View};

/// a top level navigation component used throughout the app.
/// Could extend to have state and have different links on different pages
#[component(BarComponent<G>)]
pub fn bar_component() -> View<G> {
    view! {
        div(class = "bar"){
            p{"BAR"}
        }
    }
}
