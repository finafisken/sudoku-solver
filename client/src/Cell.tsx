import { Show, type Component } from 'solid-js';

type UpdateCell = (val: number, x: number, y: number) => void

function calcStyle(x: number, y: number): string {
	const col = x + 1;
	const row = y + 1;
	const rightBorder = col % 3 == 0 && col !== 9;
	const bottomBorder = row % 3 == 0 && row !== 9;

	return `${bottomBorder ? 'border-b-4' : ''} ${rightBorder ? 'border-r-4' : ''}`
}

const Cell: Component<{ val: number, x: number, y:number, updateCell: UpdateCell}> = (props) => {
	return (
		<div class={`bg-slate-600 border-slate-700 aspect-square flex items-center border justify-center text-2xl ${calcStyle(props.x, props.y)}`}>
			<Show when={props.val > 0}>{props.val}</Show>
		</div>
	);
};

export default Cell;
