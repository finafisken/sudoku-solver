import { For, type Component } from 'solid-js';
import Cell from './Cell';

type UpdateCell = (val: number, x: number, y: number) => void

const Grid: Component<{ state: number[][], updateCell: UpdateCell }> = (props) => {
	return (
		<div class="grid grid-cols-9 gap-4">
			<For each={props.state}>
				{(row, y) => (
					<For each={row}>
						{(val, x) => (<Cell val={val} updateCell={v => props.updateCell(v, x(), y())} />)}
					</For>
				)}
			</For>
		</div>
	);
};

export default Grid;
