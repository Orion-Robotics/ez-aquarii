import classNames from "classnames";
import { JSX, Show } from "solid-js";
import { css } from "vite-plugin-inline-css-modules";

const sliderStyles = css`
  .slider {
    @apply appearance-none bg-dark-200 rounded-full h-2;
    &::-webkit-slider-thumb {
      @apply appearance-none h-4 w-4 bg-dark-100 rounded-full active:bg-dark-200 shadow-lg;
    }
    &.contrast {
      @apply bg-dark-400;
    }
  }
`;

export const BaseSlider = (
  props: {
    class?: string;
    rootClass?: string;
    label?: string;
    showValue?: boolean;
    contrast?: boolean;
  } & JSX.InputHTMLAttributes<HTMLInputElement>
) => {
  return (
    <label
      class={classNames(
        "uppercase text-xs gap-2 flex items-center",
        props.class
      )}
    >
      <span class="whitespace-nowrap">{props.label}</span>
      <input
        {...props}
        class={classNames(sliderStyles.slider, props.class)}
        classList={{ [sliderStyles.contrast]: props.contrast }}
        step={props.step || "any"}
        type="range"
      />
      <Show when={props.showValue}>
        <span>{props.value}</span>
      </Show>
    </label>
  );
};
