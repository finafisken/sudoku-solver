mod sudoku;

use axum::{
    extract::{ws, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router, Server,
};

use crate::sudoku::game::Puzzle;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/live", get(live_solve));

    Server::bind(&"0.0.0.0:1337".parse().unwrap())
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
            _ => String::default(),
        };

        let Ok(puzzle) = serde_json::from_str::<Puzzle>(&stringified_data) else {
            println!("BAD JSON: {stringified_data:?}");
            return;
        };

        let Puzzle(mut p) = puzzle;

        let solution = sudoku::game::solve(&mut p, Some(ws)).await;

        println!("{solution:?}");
    }
}
