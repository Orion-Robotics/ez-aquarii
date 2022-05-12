import classnames from "classnames";
import { Component, JSX } from "solid-js";
import { Dynamic } from "solid-js/web";

export const BaseInput: Component<
  {
    label?: string;
    class?: string;
    textarea?: boolean;
  } & JSX.InputHTMLAttributes<HTMLInputElement>
> = (props) => (
  <Dynamic
    component={props.textarea ? "textarea" : "input"}
    class={classnames(
      `
        outline-none
        p-3
        rounded-md
        bg-dark-200
        focus:outline-blue-400
        focus:outline-2
        focus:bg-dark-700
        text-xs
        block
        transition
        duration-100
      `,
      props.class
    )}
    placeholder={props.label}
    {...props}
  />
);
