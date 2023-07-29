let ws = new WebSocket("ws://localhost:3000/live");
ws.onmessage = (d) => {
    console.log(d);
};

let input = JSON.stringify(
  [
      [6, 0, 5, 7, 2, 0, 0, 3, 9],
      [4, 0, 0, 0, 0, 5, 1, 0, 0],
      [0, 2, 0, 1, 0, 0, 0, 0, 4],
      [0, 9, 0, 0, 3, 0, 7, 0, 6],
      [1, 0, 0, 8, 0, 9, 0, 0, 5],
      [2, 0, 4, 0, 5, 0, 0, 8, 0],
      [8, 0, 0, 0, 0, 3, 0, 2, 0],
      [0, 0, 2, 9, 0, 0, 0, 0, 1],
      [3, 5, 0, 0, 6, 7, 4, 0, 8]
    ]
);

// ws.send(input)
