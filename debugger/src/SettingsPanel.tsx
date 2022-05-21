import { Component, createEffect } from "solid-js";
import { createStore } from "solid-js/store";
import ArrowLeft from "~icons/mdi/arrow-left";
import { IconButton } from "./components/Base/IconButton";
import { JSONEditor } from "./components/JSONEditor";

export const SettingsPanel: Component<{
  onBack: () => void;
  onChange: (value: any) => void;
  host: string;
}> = (props) => {
  const [data, setData] = createStore({});

  createEffect(async () => {
    const value = await fetch(`http://${props.host}/config`).then((resp) =>
      resp.json()
    );
    setData(value);
  });

  return (
    <div>
      <div class="flex items-center pb-4 gap-2">
        <IconButton onClick={props.onBack}>
          <ArrowLeft />
        </IconButton>
        Robot Configuration
      </div>
      <JSONEditor
        root
        data={data}
        onChange={(path, value) => {
          setData(...path, value);
          props.onChange(data);
        }}
        structure={{
          motors: {
            speed: {
              min: 0,
              max: 1,
              step: 0.01,
            },
          },
          line: {
            pickup_threshold: {
              min: 0,
              max: 255,
              step: 1,
            },
            pickup_sensor_count: {
              min: 0,
              max: 46,
              step: 1,
            },
            trigger_threshold: {
              min: 0,
              max: 255,
              step: 1,
            },
          },
          strategy: {
            orbit: {
              curve_steepness: {
                min: 0,
                max: 5,
                step: 0.01,
              },
              shift_x: {
                min: -10,
                max: 10,
                step: 0.01,
              },
              shift_y: {
                min: 0,
                max: 10,
                step: 0.01,
              },
            },
            dampen: {
              curve_steepness: {
                min: 1,
                max: 2,
                step: 0.001,
              },
              shift_x: {
                min: -100,
                max: 100,
                step: 0.1,
              },
              shift_y: {
                min: 0,
                max: 10,
                step: 0.01,
              },
            },
          },
        }}
        path={[]}
      />
    </div>
  );
};
