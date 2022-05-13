import { Component, JSX } from "solid-js";
import { css } from "vite-plugin-inline-css-modules";
import MdiCheck from "~icons/mdi/check";

const styles = css`
  .container {
    @apply flex items-center gap-2 select-none;
  }

  .checkbox {
    @apply opacity-0 absolute h-6 w-6;

    &:checked + div {
      @apply bg-dark-400 text-white border-3;
    }
  }

  .checkmark {
    @apply border-dark-400 border-2 text-transparent rounded-md w-6 h-6 flex flex-shrink-0 justify-center items-center;
  }
`;

export const BaseCheckBox: Component<
  JSX.InputHTMLAttributes<HTMLInputElement>
> = (props) => (
  <label class={styles.container}>
    <input class={styles.checkbox} type="checkbox" {...props} />
    <div class={styles.checkmark}>
      <MdiCheck />
    </div>
  </label>
);
