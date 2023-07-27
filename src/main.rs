mod sudoku;

use axum::{
    routing::get,
    Router,
    Server,
    extract::{WebSocketUpgrade, ws},
    response::IntoResponse
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/live", get(live_solve));

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn live_solve(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(solve_and_broadcast)
}

async fn solve_and_broadcast(ws: ws::WebSocket) {
    let mut puzzle = [
        [6, 0, 5, 7, 2, 0, 0, 3, 9],
        [4, 0, 0, 0, 0, 5, 1, 0, 0],
        [0, 2, 0, 1, 0, 0, 0, 0, 4],
        [0, 9, 0, 0, 3, 0, 7, 0, 6],
        [1, 0, 0, 8, 0, 9, 0, 0, 5],
        [2, 0, 4, 0, 5, 0, 0, 8, 0],
        [8, 0, 0, 0, 0, 3, 0, 2, 0],
        [0, 0, 2, 9, 0, 0, 0, 0, 1],
        [3, 5, 0, 0, 6, 7, 4, 0, 8],
    ];

    let solution = sudoku::game::solve(&mut puzzle, Some(ws)).await;

    println!("{solution:?}");
}