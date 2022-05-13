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
    HH: 0,
    HL: 0,
    VH: 0,
    VL: 0,
    SH: 0,
    SL: 0,
  });
  const [redGain, setRedGain] = createSignal(0);
  const [blueGain, setBlueGain] = createSignal(0);

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
      [redGain, blueGain],
      async () =>
        sendJSON(`http://${props.host}/thresholds`, {
          red: redGain(),
          blue: blueGain(),
        }),
      { defer: true }
    )
  );

  createEffect(
    on(
      sliders,
      async () =>
        sendJSON(`http://${props.host}/thresholds`, {
          thresholds: Object.values(sliders()),
        }),
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
          label="Red Balance"
          class={styles.slider}
          min={0}
          max={8}
          onInput={(ev) => setRedGain(ev.currentTarget.valueAsNumber)}
        />
        <BaseSlider
          label="Blue Balance"
          class={styles.slider}
          value={blueGain()}
          onInput={(ev) => setBlueGain(ev.currentTarget.valueAsNumber)}
          min={0}
          max={8}
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
