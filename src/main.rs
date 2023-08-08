mod sudoku;

use axum::{
    extract::{ws, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router, Server,
};
use futures_util::{StreamExt, SinkExt};
use tokio::{sync::mpsc, task::JoinHandle};

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
    let (mut ws_sender, mut ws_receiver) = ws.split();
    let (tx, mut rx) = mpsc::channel::<String>(64);

    let mut current_task: Option<JoinHandle<()>> = None;

    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            ws_sender.send(ws::Message::Text(message)).await;
        }
    });
    

    while let Some(Ok(msg)) = ws_receiver.next().await {
        dbg!(&msg);
        let stringified_data = match msg {
            ws::Message::Text(t) => t,
            ws::Message::Close(_) => {
                println!("Closed socket");
                continue;
            }
            _ => String::default(),
        };

        if &stringified_data == "STOP" {
            match current_task {
                Some(ref task) => {
                    task.abort();
                    continue
                },
                None => continue,
            }
        }

        let Ok(puzzle) = serde_json::from_str::<Puzzle>(&stringified_data) else {
            println!("BAD JSON");
            continue;
        };

        let transmitter = tx.clone();

        let solve_task = tokio::spawn(async move {
            let solution = sudoku::game::solve(&mut puzzle.into(), Some(transmitter)).await;
            println!("{solution:?}");
        });

        current_task = Some(solve_task);
    }
}
