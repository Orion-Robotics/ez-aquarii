import {
  Component,
  createEffect,
  createSignal,
  For,
  on,
  onMount,
} from "solid-js";
import { css } from "vite-plugin-inline-css-modules";
import { BaseCheckBox } from "./components/Base/BaseCheckBox";
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
  const [config, setConfig] = createSignal({
    thresholds: {
      HH: 255,
      HL: 0,
      VH: 255,
      VL: 0,
      SH: 255,
      SL: 0,
    } as { [key: string]: number },
    saturation: 0,
    reset: false,
  });

  onMount(async () => {
    const resp = (await fetch(`http://${props.host}/config`, {
      method: "POST",
    }).then((res) => res.json())) as {
      thresholds: number[];
      saturation: number;
      reset: boolean;
    };
    setConfig({
      thresholds: Object.fromEntries(
        Object.entries(config().thresholds).map(([key], i) => [
          key,
          resp.thresholds[i],
        ])
      ),
      saturation: resp.saturation,
      reset: resp.reset,
    });
  });

  createEffect(
    on(
      config,
      async () => {
        sendJSON(`http://${props.host}/thresholds`, {
          thresholds: Object.values(config().thresholds),
          saturation: config().saturation,
          reset: config().reset,
        });
      },
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
        <BaseCheckBox
          label="Show Raw"
          checked={config().reset}
          onChange={(ev) =>
            setConfig({ ...config(), reset: ev.currentTarget.checked })
          }
        />
        <BaseSlider
          label="Saturation"
          class={styles.slider}
          min={0}
          max={100}
          step={1}
          onInput={(ev) =>
            setConfig({
              ...config(),
              saturation: ev.currentTarget.valueAsNumber,
            })
          }
          value={config().saturation}
        />
        <For each={Object.keys(config().thresholds)}>
          {(name) => (
            <BaseSlider
              onInput={(ev) =>
                setConfig({
                  ...config(),
                  thresholds: {
                    ...config().thresholds,
                    [name]: ev.currentTarget.valueAsNumber,
                  },
                })
              }
              label={name}
              value={config().thresholds[name]}
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
