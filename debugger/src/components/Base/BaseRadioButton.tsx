import { Component, JSX } from "solid-js";
import { Label } from "./Label";

export const BaseRadioButton: Component<
  {
    label: string;
    name?: string;
  } & JSX.InputHTMLAttributes<HTMLInputElement>
> = (props) => {
  return (
    <label class="inline-flex gap-2 items-center">
      <input
        type="radio"
        name={props.name}
        class="cursor-pointer inline-block align-middle appearance-none w-4 h-4 bg-dark-200 rounded-full checked:bg-blue-300 active:bg-blue-300 outline-blue-300 outline-2 focus:outline outline-offset-2"
        {...props}
      />
      <Label>{props.label}</Label>
    </label>
  );
};
