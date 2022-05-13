import classNames from "classnames";
import { Component, JSX } from "solid-js";
import { css } from "vite-plugin-inline-css-modules";

const styles = css`
  .iconButton {
    @apply p-1 hover:bg-white/20 rounded-full;
  }
`;

export const IconButton: Component<
  JSX.ButtonHTMLAttributes<HTMLButtonElement>
> = (props) => (
  <button class={classNames(styles.iconButton, props.class)} {...props}>
    {props.children}
  </button>
);
