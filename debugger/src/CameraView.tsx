import debounce from "lodash.debounce";
import {
  Component,
  createEffect,
  createSignal,
  For,
  on,
  onMount,
} from "solid-js";
import { css } from "vite-plugin-inline-css-modules";
import { BaseSlider } from "./components/Base/BaseSlider";

const styles = css`
  .slider {
    @apply w-40 md:w-80;
  }
`;

const sendJSON = (url: string, data: any) =>
  fetch(url, {
    method: "POST",
    body: JSON.stringify(data),
  });

export const CameraView: Component<{
  host: string;
}> = (props) => {
  const [sliders, setSliders] = createSignal<Record<string, number>>({
    HH: 255,
    HL: 0,
    VH: 255,
    VL: 0,
    SH: 255,
    SL: 0,
  });
  const [saturation, setSaturation] = createSignal(0);

  onMount(async () => {
    const resp = (
      await fetch(`http://${props.host}/get_thresholds`, {
        method: "POST",
      }).then((res) => res.json())
    ).thresholds as number[];
    setSliders(
      Object.fromEntries(
        Object.entries(sliders()).map(([key], i) => [key, resp[i]])
      )
    );
  });

  createEffect(
    on(
      [sliders, saturation],
      debounce(async () =>
        sendJSON(`http://${props.host}/thresholds`, {
          thresholds: Object.values(sliders()),
          saturation: saturation(),
        })
      ),
      { defer: true }
    )
  );

  return (
    <div class="h-full w-full flex justify-center">
      <img
        class="object-contain h-full max-w-full max-h-full"
        src={`http://${props.host}/stream.mjpg`}
      />
      <div class="absolute left-0 bottom-0 p-3 bg-black/90 rounded-tr-4 flex flex-col gap-2">
        <BaseSlider
          label="Saturation"
          class={styles.slider}
          min={0}
          max={100}
          step={1}
          onInput={(ev) => setSaturation(ev.currentTarget.valueAsNumber)}
        />
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
