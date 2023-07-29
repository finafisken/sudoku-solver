import type { Component } from 'solid-js';

type UpdateCell = (val: number) => void;

const Cell: Component<{val: number, updateCell: UpdateCell}> = (props) => {
	return (
		<div class="bg-base-300" onClick={() => props.updateCell(props.val+1)}>{props.val}</div>
	);
};

export default Cell;
