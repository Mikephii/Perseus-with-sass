mod error_views;
mod templates;
mod components;
mod api;

use perseus::prelude::*;
use sycamore::prelude::*;



#[perseus::main(api::custom_server_fn)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .template(crate::templates::about::get_template())
        .error_views(crate::error_views::get_error_views())
        .index_view(index_view_fn)
}

pub fn index_view_fn(cx:Scope) -> View<SsrNode> {
    sycamore::view! { cx,
        // We don't need a `<!DOCTYPE html>`, that's added automatically by Perseus (though that can be overriden if you really want by using `.index_view_str()`)
        // We need a `<head>` and a `<body>` at the absolute minimum for Perseus to work properly (otherwise certain script injections will fail)
        head {
            title { "Perseus Index View Example" }
            link(rel = "stylesheet", href = ".perseus/static/.styles.css")
        }
        body {
            // This creates an element into which our app will be interpolated
            // This uses a few tricks internally beyond the classic `<div id="root">`, so we use this wrapper for convenience
            PerseusRoot()
            // Because this is in the index view, this will be below every single one of our pages
            // Note that elements in here can't be selectively removed from one page, it's all-or-nothing in the index view (it wraps your whole app)
            // Note also that this won't be reloaded, even when the user switches pages
            footer { "This is a footer!" }
        }
    }
}
