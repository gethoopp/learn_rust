use warp::Filter;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::convert::Infallible;
use crate::api::handlers::{Clients, register_handler, ws_handler, unregister_clinet,unregister_client_all};

pub async fn run_server() {
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    // /register endpoint
   let register = warp::path("register")
    .and(
        // POST /register
        warp::post()
            .and(warp::body::json())
            .and(with_clients(clients.clone()))
            .and_then(register_handler)
        // OR DELETE /register/:id
        .or(
            warp::delete()
                .and(warp::path::param::<String>())
                .and(with_clients(clients.clone()))
                .and_then(unregister_clinet),
        ),
    );


    let unregister_all = warp::path("delete")
    .and(
        warp::delete()
            .and(with_clients(clients.clone()))
            .and_then(unregister_client_all),
            
    );


    // /ws/{id} endpoint
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::path::param())
        .and(with_clients(clients.clone()))
        .and_then(ws_handler);

    // root endpoint
    let root = warp::path::end().map(|| {
        "Rust WebSocket server running!"
    });

    let routes = register.or(ws_route).or(root).with(warp::cors().allow_any_origin());
    println!("Server running at http://127.0.0.1:8000");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}


