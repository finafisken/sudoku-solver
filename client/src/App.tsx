import { onMount, type Component, createSignal, Switch, Match, Show } from 'solid-js';
import Grid from './Grid';
import { createStore, produce } from 'solid-js/store';

type CellUpdate = { x: number, y: number, val: number };

const startState =
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
  ];

let ws: WebSocket;

const App: Component = () => {
  const [gridState, setGridState] = createStore<number[][]>(startState);
  // const [cellHistory, setCellHistroy] = createStore<CellUpdate[]>([]);
  const [socketReady, setSocketReady] = createSignal<boolean>(false);
  const [running, setRunning] = createSignal<boolean>(false);

  const updateCell = (val: number, x: number, y: number) => {
    setGridState(produce(grid => grid[y][x] = val))
  };

  const sendGrid = () => {
    setRunning(true)
    ws.send(JSON.stringify(gridState))
  }

  const stopSolve = () => {
    ws.send("STOP");
    setRunning(false)
  }

  onMount(() => {
    ws = new WebSocket('ws://localhost:1337/live');

    ws.onopen = () => {
      console.debug("WS open", ws);
      setSocketReady(true);
    }

    ws.onclose = () => {
      console.debug("WS close", ws);
      setSocketReady(false);
    }

    ws.onmessage = (msg) => {
      const [x, y, val] = msg.data.split(":");
      updateCell(parseInt(val), x, y);
      // setCellHistroy(produce(history => history.push({ x, y, val })));
    }
  })

  return (
    <div class="">
      <Grid state={gridState} updateCell={updateCell} />
        <button class='btn btn-primary' onClick={sendGrid}>Solve it!</button>
        <button class='btn btn-error'  onClick={stopSolve}>Stop!</button>
      { /*<input type="range" min={0} max={cellHistory.length} value={cellHistory.length} class="range" /> */}
    </div>
  );
};

export default App;
