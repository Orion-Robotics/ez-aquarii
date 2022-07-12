import * as d3 from "d3";
import { D3ZoomEvent } from "d3";
import { Component, For, JSX, onMount, Show } from "solid-js";
import { BaseSlider } from "./components/Base/BaseSlider";
import { MotorCommand, RotationDisplay } from "./components/MagnitudeDisplay";
import { CameraBlob, DataObject } from "./data_sources";
import { createStoredSignal } from "./helpers/createStoredSignal";

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

interface AnglePreference {
  color: string;
  label: string;
  distance: number;
}

const angles: AnglePreference[] = [
  {
    label: "Ball",
    color: "#ff8800",
    distance: 1.2,
  },
  {
    label: "Yellow Goal",
    color: "#fbff00",
    distance: 1.5,
  },
  {
    label: "Blue Goal",
    color: "#330ec7",
    distance: 1.5,
  },
];

const BlobComponent: Component<{
  blob?: CameraBlob;
  angleOptions: AnglePreference;
  scale: number;
}> = (props) => {
  const angle = () => props.blob?.angle;
  const distance = () => props.blob?.distance;
  return (
    <Show when={props.blob}>
      <Angle
        angle={angle()!}
        color={props.angleOptions.color}
        label={props.angleOptions.label}
        radius={distance()! * props.angleOptions.distance * props.scale}
        thickness={3}
      />
    </Show>
  );
};

export const RobotView: Component<{
  frame: DataObject;
}> = (props) => {
  const [radius, setRadius] = createStoredSignal(100, "radius");
  const [distanceScale, setDistanceScale] = createStoredSignal(
    1,
    "distance_scale"
  );
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
          <BlobComponent
            blob={props.frame.camera_data.ball!}
            angleOptions={angles[0]}
            scale={distanceScale()}
          />
          <BlobComponent
            blob={props.frame.camera_data.yellow_goal!}
            angleOptions={angles[1]}
            scale={distanceScale()}
          />
          <BlobComponent
            blob={props.frame.camera_data.blue_goal!}
            angleOptions={angles[2]}
            scale={distanceScale()}
          />
          <For each={props.frame.line_detections}>
            {(line_detection, i) => {
              const circleSize = () => radius() * 0.05;
              const angle = () => {
                const trig_angle =
                  Math.PI * 2 * (i() / props.frame.line_detections.length);
                return trig_angle - Math.PI;
              };
              const fillColor = () => (line_detection ? "#ff3d3d" : "#737373");
              return (
                <circle
                  cx={-radius() * Math.cos(angle())}
                  cy={-radius() * -Math.sin(angle())}
                  r={circleSize()}
                  fill={fillColor()}
                />
              );
            }}
          </For>
          <g>
            <Show when={typeof props.frame.initial_orientation === "number"}>
              <Angle
                angle={props.frame.initial_orientation!}
                color="#03bafc"
                label="Initial Orientation"
                radius={radius() * 0.5}
                thickness={radius() * 0.03}
              />
              <Angle
                angle={props.frame.data.orientation!}
                color="#7aa2ff"
                label="Rotation"
                radius={radius() * 0.7}
                thickness={radius() * 0.03}
              />
            </Show>
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
            <Show when={props.frame.move_vector}>
              <Line
                endX={radius() * 0.9 * props.frame.move_vector!.x}
                endY={radius() * 0.9 * -props.frame.move_vector!.y}
                offset={10}
                color="#5cff88"
                thickness={radius() * 0.03}
                label="Move Vector"
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
            <Show when={props.frame.strategy.type === "Orbit"}>
              <Angle
                angle={props.frame.strategy.orbit_angle}
                color="#b56bff"
                label="After Dampen"
                radius={radius() * 1.5}
                thickness={radius() * 0.03}
              />
              <Angle
                angle={props.frame.strategy.before_dampen_angle}
                color="#fcba03"
                label="Before Dampen"
                radius={radius() * 1.2}
                thickness={radius() * 0.03}
              />
            </Show>
          </g>
        </g>
      </svg>
      <div class="text-xs absolute top-0 left-0 bg-black/80 p-3 rounded-br-4">
        <For each={Object.keys(props.frame.tick_rates)}>
          {(key) => (
            <div class="flex">
              <span class="w-24">{key}</span>
              <span>{props.frame.tick_rates[key]}</span>
            </div>
          )}
        </For>
      </div>
      <div class="absolute bottom-0 right-0 bg-black/80 p-3 rounded-tl-4 w-60 flex flex-col items-center gap-2">
        <p class="text-sm uppercase">Motor Commands</p>
        <MotorCommand command={props.frame.motor_commands[0]} />
        <MotorCommand command={props.frame.motor_commands[1]} />
        <MotorCommand command={props.frame.motor_commands[2]} />
        <MotorCommand command={props.frame.motor_commands[3]} />
        <p class="text-sm uppercase">Rotation</p>
        <RotationDisplay magnitude={props.frame.scaled_rotation} />
      </div>
      <div class="absolute top-0 left-1/2 bg-black/80 p-3 rounded-b-4 transform -translate-x-1/2">
        <h1 class="font-bold">
          Current State:{" "}
          <span class="text-green-400">{props.frame.strategy.type}</span>
        </h1>
      </div>
      <div class="absolute bottom-0 left-0 bg-black/80 p-3 rounded-tr-4 flex flex-col gap-2">
        <BaseSlider
          type="range"
          label="Radius"
          showValue
          step={1}
          max={1000}
          value={radius()}
          onInput={(ev) => setRadius(+ev.currentTarget.value)}
        />
        <BaseSlider
          type="range"
          label="Distance Scale"
          showValue
          max={2}
          step={0.01}
          value={distanceScale()}
          onInput={(ev) => setDistanceScale(+ev.currentTarget.value)}
        />
      </div>
    </div>
  );
};
