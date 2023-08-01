mod sudoku;

use axum::{
    extract::{ws, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router, Server,
};
use futures_util::StreamExt;

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

async fn solve_and_broadcast(ws: ws::WebSocket) {
    let (mut sender, mut receiver) = ws.split();
    let mut _recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            dbg!(&msg);
            let stringified_data = match msg {
                ws::Message::Text(t) => t,
                _ => String::default(),
            };

            let Ok(puzzle) = serde_json::from_str::<Puzzle>(&stringified_data) else {
                println!("BAD JSON");
                return;
            };

            let solution = sudoku::game::solve(&mut puzzle.into(), Some(&mut sender)).await;

            println!("{solution:?}");
        }
    });
}
