import classNames from "classnames";
import { Component, JSX } from "solid-js";
import { Dynamic } from "solid-js/web";
import { css } from "vite-plugin-inline-css-modules";

const styles = css`
  .input {
    @apply outline-none p-3 rounded-md bg-dark-200
      focus:outline-blue-400 focus:outline-2 focus:bg-dark-700
      text-xs block transition duration-100;
  }
`;

export const BaseInput: Component<
  {
    label?: string;
    class?: string;
    textarea?: boolean;
  } & JSX.InputHTMLAttributes<HTMLInputElement>
> = (props) => (
  <Dynamic
    {...props}
    component={props.textarea ? "textarea" : "input"}
    class={classNames(styles.input, props.class)}
    placeholder={props.label}
  />
);
