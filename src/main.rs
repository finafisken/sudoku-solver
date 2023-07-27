mod sudoku;

use axum::{
    routing::get,
    Router,
    Server,
    extract::{WebSocketUpgrade, ws},
    response::IntoResponse
};

use crate::sudoku::game::Puzzle;

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

async fn solve_and_broadcast(mut ws: ws::WebSocket) {
    if let Some(msg) = ws.recv().await {
        let Ok(data) = msg else {
            println!("Bad msg!");
            return;
        };

        let stringified_data = match data {
            ws::Message::Text(t) => t,
            _ => String::default()
        };

        let Ok(puzzle) = serde_json::from_str::<Puzzle>(&stringified_data) else {
            println!("BAD JSON");
            return;
        };

        let mut p = puzzle.grid;

        let solution = sudoku::game::solve(&mut p, Some(ws)).await;

        println!("{solution:?}");
    }
}