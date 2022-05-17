import { Component, createSignal } from "solid-js";
import { Vec2 } from "../data_sources";

export const VectorPicker: Component<{
  max: number;
  onChange?: (value: Vec2) => void;
  value: Vec2;
}> = (props) => {
  let root: SVGSVGElement;
  const endX = () => props.value.x;
  const endY = () => props.value.y;

  const [dragging, setDragging] = createSignal(false);

  const onMove = (e: MouseEvent) => {
    if (!dragging()) return;
    const { clientX, clientY } = e;
    const { left, top, width, height } = root.getBoundingClientRect();
    const relX = clientX - left;
    const relY = clientY - top;
    const x = relX - width / 2;
    const y = relY - height / 2;
    console.log({ x, y });
  };

  return (
    <svg
      class="bg-dark-900"
      style="aspect-ratio: 1"
      viewBox="0 0 100 100"
      onMouseUp={() => setDragging(false)}
      onMouseMove={onMove}
      ref={root!}
    >
      <g>
        <line
          x1={50}
          y1={0}
          x2={50}
          y2={100}
          stroke="white"
          stroke-width={0.1}
        ></line>
        <line
          x1={0}
          y1={50}
          x2={100}
          y2={50}
          stroke="white"
          stroke-width={0.1}
        ></line>
        <circle
          cx={50}
          cy={50}
          stroke="white"
          stroke-width={0.1}
          fill="transparent"
          r={5}
        ></circle>
        <circle
          cx={endX()}
          cy={endY()}
          stroke="#8fb8ff"
          stroke-width={1}
          fill="#8fb8ff40"
          r={5}
          class="cursor-grab"
          onMouseDown={() => setDragging(true)}
        ></circle>
      </g>
    </svg>
  );
};
