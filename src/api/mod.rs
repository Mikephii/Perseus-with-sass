use perseus::engine_only_fn;
use serde::{Deserialize, Serialize};
use std::error::Error;

// Note: we use fully-qualified paths in the types to this function so we don't
// have to target-gate some more imports
#[cfg(engine)] // We only have access to `warp` etc. on the engine-side, so this function should only exist tehre
pub async fn custom_server_fn<
    M: perseus::stores::MutableStore + 'static,
    T: perseus::i18n::TranslationsManager + 'static,
>(
    turbine: &'static perseus::turbine::Turbine<M, T>,
    opts: perseus::server::ServerOptions,
    (host, port): (String, u16),
) {
    use perseus_warp::perseus_routes;
    use std::net::SocketAddr;
    use warp::Filter;

    // The Warp integration takes a `SocketAddr`, so we convert the given host and
    // port into that format
    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid address provided to bind to.");
    // Now, we generate the routes from the properties we were given
    // All integrations provide some function for setting them up that just takes
    // those universal properties Usually, you shouldn't ever have to worry
    // about the value of the properties, which are set from your `PerseusApp`
    // config
    let perseus_routes = perseus_routes(turbine, opts).await;
    // And now set up our own routes
    // You could set up as many of these as you like in a production app
    // Note that they mustn't define anything under `/.perseus` or anything
    // conflicting with any of your static aliases This one will just echo
    // whatever is sent to it
    let api_route = warp::path!("api" / "echo" / String).map(|msg| {
        // You can do absolutely anything in here that you can do with Warp as usual
        msg
    });
    // We could add as many routes as we wanted here, but the Perseus routes, no
    // matter what integration you're using, MUST always come last! This is
    // because they define a wildcard handler for pages, which has to be defined
    // last, or none of your routes will do anything.
    let routes = api_route.or(perseus_routes);

    warp::serve(routes).run(addr).await;

    // If you try interacting with the app as usual, everything will work fine
    // If you try going to `/api/echo/test`, you'll get `test` printed back to
    // you! Try replacing `test` with anything else and it'll print whatever
    // you put in back to you!
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    title: String,
    description: String,
    image: SbImage,
}

#[derive(Serialize, Deserialize)]
pub struct SbImage {
    id: u32,
    alt: String,
    name: String,
    focus: String,
    title: String,
    filename: String,
    copyright: String,
    fieldtype: String,
    is_external_url: bool,
}

#[engine_only_fn]
pub async fn get_posts() -> Result<Vec<Post>, reqwest::Error> {
    let url = "https://api.storyblok.com/v2/cdn/stories?token=T1lVJToB5V7fQxD0f4nRPQtt&version=draft&starts_with=portfolio-items";
    let json: serde_json::Value = reqwest::get(url).await?.json().await?;

    if let Some(stories) = json["stories"].as_array() {
        let posts: Vec<Post> = stories
            .iter()
            .map(|story| -> Post { serde_json::from_value(story["content"]).unwrap() })
            .collect();

        return Ok(posts);
    }
    // TODO: custom error
}
