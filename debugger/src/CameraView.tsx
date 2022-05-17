import {
  Component,
  createEffect,
  createSignal,
  For,
  on,
  onMount,
  Show,
} from "solid-js";
import { css } from "vite-plugin-inline-css-modules";
import ArrowLeft from "~icons/mdi/arrow-left";
import ArrowRight from "~icons/mdi/arrow-right";
import { BaseCheckBox } from "./components/Base/BaseCheckBox";
import { BaseSlider } from "./components/Base/BaseSlider";
import { IconButton } from "./components/Base/IconButton";

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

interface Thresholds {
  HH: number;
  HL: number;
  VH: number;
  VL: number;
  SH: number;
  SL: number;
  [key: string]: number;
}

interface CameraConfig {
  thresholds: Thresholds[];
  camera: {
    saturation: number;
  };
  bypass: boolean;
}

const serialize = (config: CameraConfig) => {
  return {
    ...config,
    thresholds: config.thresholds.map((threshold) => Object.values(threshold)),
  };
};

const thresholdsFromArray = (arr: number[]): Thresholds => {
  const [HH, HL, VH, VL, SH, SL] = arr;
  return {
    HH,
    HL,
    VH,
    VL,
    SH,
    SL,
  };
};

const ThresholdEditor: Component<{
  thresholds: Thresholds;
  onChange: (threshold: Thresholds) => void;
}> = (props) => {
  return (
    <For each={Object.keys(props.thresholds)}>
      {(name) => (
        <BaseSlider
          onInput={(ev) =>
            props.onChange({
              ...props.thresholds,
              [name]: ev.currentTarget.valueAsNumber,
            })
          }
          label={name}
          value={props.thresholds[name]}
          class={styles.slider}
          min={0}
          max={255}
          step={1}
        />
      )}
    </For>
  );
};

const pages = ["Camera Settings", "Ball", "Yellow Goal", "Blue Goal"];

export const CameraView: Component<{
  host: string;
}> = (props) => {
  const [page, setPage] = createSignal(0);
  const [config, setConfig] = createSignal<CameraConfig>({
    thresholds: [
      { HH: 255, HL: 0, VH: 255, VL: 0, SH: 255, SL: 0 },
      { HH: 255, HL: 0, VH: 255, VL: 0, SH: 255, SL: 0 },
      { HH: 255, HL: 0, VH: 255, VL: 0, SH: 255, SL: 0 },
    ] as Thresholds[],
    camera: {
      saturation: 0,
    },
    bypass: false,
  });
  const pageCount = () => pages.length;

  onMount(async () => {
    const resp = (await fetch(`http://${props.host}/get_config`, {
      method: "POST",
    }).then((res) => res.json())) as {
      thresholds: number[][];
      camera: {
        saturation: number;
      };
      bypass: boolean;
    };
    setConfig({
      ...resp,
      thresholds: resp.thresholds.map((arr) => thresholdsFromArray(arr)),
    });
  });

  createEffect(
    on(
      page,
      async () => {
        sendJSON(`http://${props.host}/page`, { page: page() });
      },
      { defer: true }
    )
  );

  createEffect(
    on(
      config,
      () => {
        sendJSON(`http://${props.host}/config`, serialize(config()));
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
        <span class="text-sm uppercase">{pages[page()]}</span>
        <Show when={page() === 0}>
          <BaseCheckBox
            checked={config().bypass}
            label="Bypass"
            onChange={(ev) =>
              setConfig({ ...config(), bypass: ev.currentTarget.checked })
            }
          />
          <BaseSlider
            label="Saturation"
            min={0}
            max={100}
            step={1}
            class={styles.slider}
            showValue
            onInput={(ev) =>
              setConfig({
                ...config(),
                camera: {
                  ...config().camera,
                  saturation: ev.currentTarget.valueAsNumber,
                },
              })
            }
          />
        </Show>
        <Show when={page() > 0}>
          <ThresholdEditor
            thresholds={config().thresholds[page() - 1]}
            onChange={(threshold) => {
              const thresholds = [...config().thresholds];
              thresholds[page() - 1] = threshold;
              setConfig({
                ...config(),
                thresholds,
              });
            }}
          />
        </Show>
        <div class="text-xs flex gap-2 items-center">
          <IconButton onClick={() => setPage(Math.max(0, page() - 1))}>
            <ArrowLeft />
          </IconButton>
          <IconButton
            onClick={() => setPage(Math.min(pageCount() - 1, page() + 1))}
          >
            <ArrowRight />
          </IconButton>
          <span>
            Page {page() + 1} / {pageCount()}
          </span>
        </div>
      </div>
    </div>
  );
};
