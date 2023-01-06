use sycamore::prelude::*;

/// a top level navigation component used throughout the app.
/// Could extend to have state and have different links on different pages
#[component]
pub fn cmp<G:Html>(cx:Scope) -> View<G> {
    view! {cx,
        div(class = "bar"){
            p{"BAR"}
        }
    }
}
