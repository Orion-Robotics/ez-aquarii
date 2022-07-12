import { Component } from "solid-js";
import { css } from "vite-plugin-inline-css-modules";

const classes = css`
  .command {
    @apply h-4 bg-blue-500 rounded-sm;
  }
`;

export const MagnitudeDisplay: Component<{
  percent: number;
}> = (props) => {
  const percentCss = () => props.percent * 100;

  return (
    <div
      class={classes.command}
      style={`width: ${Math.abs(
        percentCss() / 2
      )}%; margin-left: ${percentCss()}px`}
    />
  );
};

export const MotorCommand: Component<{
  command: number;
}> = (props) => {
  return <MagnitudeDisplay percent={(props.command - 127) / 127 / 2} />;
};

export const RotationDisplay: Component<{
  magnitude: number;
}> = (props) => {
  return <MagnitudeDisplay percent={props.magnitude} />;
};
