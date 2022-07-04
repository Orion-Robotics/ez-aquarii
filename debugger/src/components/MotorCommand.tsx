import { Component } from "solid-js";
import { css } from "vite-plugin-inline-css-modules";

const classes = css`
  .command {
    @apply h-4 bg-blue-500 mb-2 rounded-sm;
  }
`;

export const MotorCommand: Component<{
  command: number;
}> = (props) => {
  const percent = () => (100 * ((props.command - 127) / 127)) / 2;
  return (
    <div
      class={classes.command}
      style={`width: ${Math.abs(percent() / 2)}%; margin-left: ${percent()}px`}
    />
  );
};
