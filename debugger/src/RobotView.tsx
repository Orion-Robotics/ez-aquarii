import * as d3 from "d3";
import { D3ZoomEvent } from "d3";
import { Component, For, JSX, onMount, Show } from "solid-js";
import { BaseSlider } from "./Base/BaseSlider";
import { DataObject } from "./data_sources";
import { createStoredSignal } from "./Helpers/createStoredSignal";

const Line: Component<
  {
    endX: number;
    endY: number;
    offset: number;
    thickness: number;
    color: string;
    label: string;
  } & JSX.LineSVGAttributes<SVGLineElement>
> = (props) => {
  return (
    <>
      <marker
        id="triangle"
        viewBox="0 0 10 10"
        refX="1"
        refY="5"
        markerUnits="strokeWidth"
        markerWidth="10"
        markerHeight="10"
        orient="auto"
      ></marker>
      <text
        x={props.endX + props.offset}
        y={props.endY + props.offset}
        fill={props.color}
        font-size="8px"
      >
        {props.label}
      </text>
      <line
        x1={0}
        y1={0}
        x2={props.endX}
        y2={props.endY}
        stroke={props.color}
        stroke-width={props.thickness}
        stroke-linecap="round"
        {...props}
      />
    </>
  );
};

const Angle: Component<{
  radius: number;
  angle: number;
  thickness: number;
  color: string;
  label: string;
}> = (props) => {
  return (
    <Line
      endX={props.radius * Math.cos(props.angle)}
      endY={props.radius * -Math.sin(props.angle)}
      offset={10}
      stroke-dasharray="1 6"
      {...props}
    />
  );
};

const gridSize = 60;
const gridDotSize = 5;

export const RobotView: Component<{
  frame: DataObject;
}> = (props) => {
  const [radius, setRadius] = createStoredSignal(100, "radius");
  let container: SVGSVGElement;
  let g: SVGGElement;
  let dots: SVGPatternElement;

  onMount(() => {
    const svg = d3.select(container!);
    const content = d3.select(g!);
    const dotsSelection = d3.select(dots!);
    let zoom = d3
      .zoom<SVGSVGElement, unknown>()
      .on("zoom", ({ transform }: D3ZoomEvent<SVGSVGElement, unknown>) => {
        content.attr("transform", transform.toString());
        // update grid
        dotsSelection
          .attr("x", transform.x)
          .attr("y", transform.y)
          .attr("width", gridSize * transform.k)
          .attr("height", gridSize * transform.k)
          .selectAll("rect")
          .attr("x", (gridSize * transform.k) / 2 - gridDotSize / 2)
          .attr("y", (gridSize * transform.k) / 2 - gridDotSize / 2)
          .attr("opacity", Math.min(transform.k, 1)); // Lower opacity as the pattern gets more dense.
      });
    svg.call(zoom);
    zoom.translateTo(svg, 0, 0);
  });

  return (
    <div class="relative h-full w-full">
      <svg class="w-full h-full" ref={container!}>
        <pattern
          ref={dots!}
          id="dot-pattern"
          patternUnits="userSpaceOnUse"
          width={gridSize}
          height={gridSize}
        >
          <rect
            width={gridDotSize}
            height={gridDotSize}
            fill="#a4a4a450"
            x="11"
            y="11"
            opacity="1"
          ></rect>
        </pattern>
        <rect fill="url(#dot-pattern)" width="100%" height="100%"></rect>
        <g ref={g!}>
          <circle
            cx={0}
            cy={0}
            r={radius()}
            stroke="white"
            stroke-width={radius() * 0.03}
            fill={"transparent"}
          />
          <For each={props.frame.line_detections}>
            {(line_detection, i) => {
              const circleSize = () => radius() * 0.05;
              const angle = () =>
                Math.PI * 2 * (i() / props.frame.line_detections.length);
              const fillColor = () => (line_detection ? "#ff3d3d" : "#737373");
              return (
                <circle
                  cx={radius() * Math.cos(angle())}
                  cy={radius() * -Math.sin(angle())}
                  r={circleSize()}
                  fill={fillColor()}
                />
              );
            }}
          </For>
          <g>
            <Show when={props.frame.line_vector}>
              <Line
                endX={radius() * 0.7 * props.frame.line_vector!.x}
                endY={radius() * 0.7 * -props.frame.line_vector!.y}
                offset={10}
                color="#ffaf75"
                thickness={radius() * 0.03}
                label="Line Vector"
              />
            </Show>
            <Show when={props.frame.previous_vec}>
              <Line
                endX={radius() * 0.4 * props.frame.previous_vec!.x}
                endY={radius() * 0.4 * -props.frame.previous_vec!.y}
                offset={10}
                color="#bfff80"
                thickness={radius() * 0.03}
                label="Previous Line Vector"
              />
            </Show>
            <Angle
              angle={props.frame.orbit_angle}
              color="#b56bff"
              label="Orbit Angle"
              radius={radius() * 1.5}
              thickness={radius() * 0.03}
            />
            <Show when={props.frame.ball_follow_vector}>
              {(vector) => {
                const endX = () => radius() * 2 * vector.x;
                const endY = () => radius() * 2 * -vector.y;
                return (
                  <>
                    <Line
                      endX={endX()}
                      endY={endY()}
                      color="#34ebe8"
                      label="Ball Follow Vector"
                      offset={10}
                      thickness={radius() * 0.02}
                      stroke-dasharray="1 6"
                    />
                    <circle
                      cx={endX()}
                      cy={endY()}
                      r={radius() * 0.08}
                      fill="#34ebe8"
                    />
                  </>
                );
              }}
            </Show>
          </g>
        </g>
      </svg>
      <div class="absolute bottom-3 left-3">
        <BaseSlider
          type="range"
          label="Radius"
          showValue
          step={1}
          max={1000}
          value={radius()}
          onInput={(ev) => setRadius(+ev.currentTarget.value)}
        />
      </div>
    </div>
  );
};
