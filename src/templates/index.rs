use crate::components::bar;
use crate::{storyblok, storyblok::utility_types::SbImage};
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "IndexPageStateRx")]
pub struct IndexPageState {
    pub greeting: String,
}

#[auto_scope]
pub fn index_page(cx: Scope, state: &IndexPageStateRx) -> View<G> {
    view! { cx,
        bar::cmp()
        p(class = "test-class") { (state.greeting.get()) }
        a(href = "about", id = "about-link") { "About!" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index")
        .build_state_fn(get_build_state)
        .view_with_state(index_page)
        .head_with_state(head)
        .build()
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub title: String,
    pub description: String,
    pub image: SbImage,
}

#[engine_only_fn]
pub async fn get_build_state(
    _info: StateGeneratorInfo<()>,
) -> Result<IndexPageState, BlamedError<storyblok::errors::MyCustomError>> {
    let mut posts: Vec<Post> = storyblok::get_content().await?;

    Ok(IndexPageState {
        greeting: posts.remove(0).title,
    })
}

#[engine_only_fn]
pub fn head(cx: Scope, _props: IndexPageState) -> View<SsrNode> {
    view! { cx,
        title{ "Index Page | Perseus Example"}
    }
}
