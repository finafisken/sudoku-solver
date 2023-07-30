import { For, type Component } from 'solid-js';
import Cell from './Cell';

type UpdateCell = (val: number, x: number, y: number) => void

const Grid: Component<{ state: number[][], updateCell: UpdateCell }> = (props) => {

	return (
		<div class="grid grid-cols-9 grid-rows-9 rounded-lg overflow-hidden max-w-lg border-4 border-slate-700">
			<For each={props.state}>
				{(row, y) => (
					<For each={row}>
						{(val, x) => (<Cell val={val} x={x()} y={y()} updateCell={props.updateCell} />)}
					</For>
				)}
			</For>
		</div>
	);
};

export default Grid;
