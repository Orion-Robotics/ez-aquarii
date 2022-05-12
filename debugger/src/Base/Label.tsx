import classNames from "classnames";
import { Component } from "solid-js";

export const Label: Component<{ class?: string }> = (props) => {
  return (
    <span
      {...props}
      class={classNames("uppercase text-xs text-gray-300", props.class)}
    >
      {props.children}
    </span>
  );
};
