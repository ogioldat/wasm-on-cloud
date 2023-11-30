use http::{Request, Response};
use spin_sdk::{http::IntoResponse, http_component};

/// A simple Spin HTTP component.

/// A simple Spin HTTP component.
#[http_component]
async fn handle_hello_rust(_req: Request<()>) -> anyhow::Result<impl IntoResponse> {
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello, Fermyon")?)
}

// #[derive(Debug, Serialize, Clone)]
// struct MenuItem {
//     name: String,spin
//     price: f32,
// }

// fn get_menu_items(_req: Request<()>, _params: Params) -> anyhow::Result<Response<()>> {
//     Ok(http::Response::builder().status(200).body(Some("".into())))

//     ResponseBuilder::new(StatusCode.ok)
//     .build()
// }

// struct Health {}

// fn get_health(_req: Request<()>, _params: Params) -> anyhow::Result<Response<()>> {
//     // Ok(http::Response::builder()
//     //     .status(200)
//     //     .body(Some("".into()))?)

//     Ok(http::Response::builder().status(204).body(None).unwrap())
// }

// fn handle_not_found(_req: Request<()>, _params: Params) -> anyhow::Result<Response<()>> {
//     not_found()
// }
