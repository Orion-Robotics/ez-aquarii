import { Component, createEffect, createSignal, For, on } from "solid-js";
import { css } from "vite-plugin-inline-css-modules";
import { BaseSlider } from "./components/Base/BaseSlider";

const styles = css`
  .slider {
    @apply w-40 md:w-80;
  }
`;

export const CameraView: Component<{
  host: string;
}> = (props) => {
  const [sliders, setSliders] = createSignal<Record<string, number>>({
    HH: 0,
    HL: 0,
    VH: 0,
    VL: 0,
    SH: 0,
    SL: 0,
  });

  createEffect(
    on(
      sliders,
      async () => {
        await fetch(`http://${props.host}/thresholds`, {
          method: "POST",
          body: JSON.stringify({
            thresholds: Object.values(sliders()),
          }),
        });
      },
      { defer: true }
    )
  );

  return (
    <div class="h-full">
      <img
        class="object-contain h-full max-w-full max-h-full"
        src={`http://${props.host}/stream.mjpg`}
      />
      <div class="absolute left-0 bottom-0 p-3 bg-black/90 rounded-tr-4 flex flex-col gap-2">
        <For each={Object.keys(sliders())}>
          {(name) => (
            <BaseSlider
              onInput={(ev) =>
                setSliders({
                  ...sliders(),
                  [name]: ev.currentTarget.valueAsNumber,
                })
              }
              label={name}
              value={sliders()[name]}
              class={styles.slider}
              min={0}
              max={255}
              step={1}
            />
          )}
        </For>
      </div>
    </div>
  );
};
