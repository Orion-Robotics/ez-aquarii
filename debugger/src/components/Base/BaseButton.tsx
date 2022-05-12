import { Component, JSX } from "solid-js";
import { css } from "vite-plugin-inline-css-modules";
import { Shimmer } from "./shimmer";

const BaseButtonStyles = css`
  .button {
    @apply px-4 py-2 rounded-md bg-blue-500 hover:bg-blue-400 hover:shadow-md active:shadow-xl outline-none focus:outline-blue-400 focus:outline-2;
  }
`;

export const BaseButton: Component<
  {
    class?: string;
    square?: boolean;
  } & JSX.ButtonHTMLAttributes<HTMLButtonElement>
> = (props) => {
  return (
    <button class={BaseButtonStyles.button} {...props}>
      {props.children}
    </button>
  );
};

export const ShimmerButton: Component<
  JSX.ButtonHTMLAttributes<HTMLButtonElement>
> = (props) => {
  return (
    <Shimmer>
      <BaseButton {...props} />
    </Shimmer>
  );
};
