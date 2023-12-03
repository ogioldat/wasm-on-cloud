mod api;

use api::{get_menu, place_order};
use spin_sdk::{
    http::{Request, Response, Router},
    http_component,
};

#[http_component]
fn handle_route(req: Request) -> Response {
    let mut router = Router::new();
    router.get("/menu", get_menu);
    router.post("/order", place_order);
    router.handle(req)
}
