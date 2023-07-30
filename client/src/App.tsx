import { onMount, type Component, createSignal } from 'solid-js';
import Grid from './Grid';
import { createStore, produce } from 'solid-js/store';

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

const ws = new WebSocket('ws://localhost:1337/live');
ws.onmessage = (d) => console.log(d);

const App: Component = () => {
  const [gridState, setGridState] = createStore(startState);
  const [readyToSolve, setReadyToSolve] = createSignal(true);

  const updateCell = (val: number, x: number, y: number) => {
    setGridState(produce(grid => grid[y][x] = val))
  };

  const onClick = () => {
    ws.send(JSON.stringify(gridState))
    setReadyToSolve(false)
  }

  onMount(() => {
    ws.onmessage = (msg) => {
      console.log(msg.data);
      const [x, y, val] = msg.data.split(":");
      updateCell(val, x, y);
    }
  })

  return (
    <div class="">
      <Grid state={gridState} updateCell={updateCell} />
      <button class='btn btn-primary' disabled={!readyToSolve()} onClick={onClick}>Solve it!</button>
    </div>
  );
};

export default App;
