let ws = new WebSocket("ws://localhost:3000/live");
ws.onmessage = (d) => {
    console.log(d);
};