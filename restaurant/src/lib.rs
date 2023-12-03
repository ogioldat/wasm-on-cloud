mod api;

use api::{
    menu::get_menu,
    order::{check_order, place_order},
};
use spin_sdk::{
    http::{Request, Response, Router},
    http_component,
};

#[http_component]
fn handle_route(req: Request) -> Response {
    let mut router = Router::new();
    router.get("/menu", get_menu);
    router.post("/order/:name", place_order);
    router.get("/order/:id", check_order);
    router.handle(req)
}
