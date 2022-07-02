import { Component, createSignal, Match, Show, Switch } from "solid-js";
import { css } from "vite-plugin-inline-css-modules";
import ArrowLeft from "~icons/mdi/arrow-left";
import ArrowRight from "~icons/mdi/arrow-right";
import SettingsCog from "~icons/mdi/cog";
import { CameraView } from "./CameraView";
import { ShimmerButton } from "./components/Base/BaseButton";
import { BaseInput } from "./components/Base/BaseInput";
import { BaseRadioButton } from "./components/Base/BaseRadioButton";
import { BaseSlider } from "./components/Base/BaseSlider";
import { IconButton } from "./components/Base/IconButton";
import { Label } from "./components/Base/Label";
import { Split } from "./components/Base/Split";
import {
  DataObject,
  DataSource,
  ServerSource,
  TextSource,
} from "./data_sources";
import { createStoredSignal } from "./helpers/createStoredSignal";
import { RobotView } from "./RobotView";
import { SettingsPanel } from "./SettingsPanel";

const styles = css`
  .gutter {
    @apply bg-dark-800 active:bg-blue-500 transition duration-200;
    &.vertical {
      @apply cursor-ew-resize w-0.5;
    }
    &.horizontal {
      @apply h-1 cursor-ns-resize;
    }
  }
`;

enum View {
  Robot,
  Camera,
  Config,
}

const App: Component = () => {
  const [serverAddress, setServerAddress] = createStoredSignal(
    "0.0.0.0:7272",
    "serverAddress"
  );
  const [textData, setTextData] = createStoredSignal("", "textData");
  const [sourceType, setSourceType] = createStoredSignal<"server" | "text">(
    "server",
    "sourceType"
  );
  const [view, setView] = createStoredSignal<View>(View.Robot, "view");
  const [started, setStarted] = createSignal(false);
  const [frameCount, setFrameCount] = createSignal(0);
  const [frameIndex, setFrameIndex] = createSignal(0);
  const [currentFrame, setCurrentFrame] = createSignal<DataObject | undefined>(
    undefined
  );
  const [editingSettings, setEditingSettings] = createSignal(false);

  let source: DataSource | undefined = undefined;

  const onConfigChange = (value: any) => {
    if (source instanceof ServerSource) {
      source.ws.send(JSON.stringify(value));
    }
  };

  const onClick = () => {
    setStarted(!started());
    if (started()) {
      switch (sourceType()) {
        case "server":
          source = new ServerSource(serverAddress());
          break;
        case "text":
          source = new TextSource(textData());
      }
      source!.onFrame((frame) => {
        setFrameCount(source!.numFrames() - 1);
        setFrameIndex(source!.currentFrame());
        setCurrentFrame(frame);
      });
      source.next();
    } else {
      source!.stop();
    }
  };

  return (
    <div class="flex h-full">
      <Split
        sizes={[30, 70]}
        gutterSize={5}
        direction="horizontal"
        gutterClass={`${styles.gutter} ${styles.vertical}`}
      >
        <div class="bg-dark-600 h-full overflow-auto">
          <div class="px-4 py-4 flex flex-col gap-4">
            <Show when={!editingSettings()}>
              <div class="flex items-center">
                <a class="hover:underline underline-offset-4 w-full select-none">
                  Nebula
                </a>
                <IconButton onClick={() => setEditingSettings(true)}>
                  <SettingsCog />
                </IconButton>
              </div>
              <BaseRadioButton
                label="Server"
                name="source"
                checked={sourceType() === "server"}
                onClick={() => setSourceType("server")}
              />
              <BaseRadioButton
                label="Text Input"
                name="source"
                checked={sourceType() === "text"}
                onClick={() => setSourceType("text")}
              />
              <Switch fallback={<p>Select a Data Source</p>}>
                <Match when={sourceType() === "server"}>
                  <Label>Server Address</Label>
                  <BaseInput
                    label="0.0.0.0:7272"
                    onInput={(ev) => setServerAddress(ev.currentTarget.value)}
                    value={serverAddress()}
                  />
                </Match>
                <Match when={sourceType() === "text"}>
                  <Label>Text Input</Label>
                  <BaseInput
                    textarea
                    onInput={(ev) => setTextData(ev.currentTarget.value)}
                    value={textData()}
                  />
                </Match>
              </Switch>
              <ShimmerButton onClick={onClick}>
                {started() ? "Stop" : "Start"}
              </ShimmerButton>
            </Show>
            <Show when={editingSettings()}>
              <SettingsPanel
                onBack={() => setEditingSettings(false)}
                onChange={onConfigChange}
                host={serverAddress()}
              />
            </Show>
          </div>
        </div>
        <div class="flex-1 flex-col flex">
          <Show when={started()}>
            <div class="bg-dark-300 p-2 flex gap-2">
              <ShimmerButton onClick={() => source?.back()}>
                <ArrowLeft />
              </ShimmerButton>
              <ShimmerButton onClick={() => source?.next()}>
                <ArrowRight />
              </ShimmerButton>
              <BaseSlider
                class="w-full"
                contrast
                showValue
                label="Frame No."
                min={0}
                max={frameCount()}
                step={1}
                value={frameIndex()}
                onInput={(ev) => {
                  source?.goTo(ev.currentTarget.valueAsNumber);
                  setFrameIndex(ev.currentTarget.valueAsNumber);
                }}
              />
            </div>
            <Split
              direction="vertical"
              gutterSize={5}
              sizes={[70, 30]}
              gutterClass={`${styles.gutter} ${styles.horizontal}`}
            >
              <div class="relative">
                <Switch>
                  <Show
                    when={currentFrame()}
                    fallback={<div class="flex-1 p-3">No frames</div>}
                  >
                    <Match when={view() === View.Robot}>
                      <RobotView frame={currentFrame()!} />
                    </Match>
                  </Show>
                  <Match when={view() === View.Camera}>
                    <CameraView host={serverAddress()} />
                  </Match>
                </Switch>
                <div class="absolute top-0 right-0 flex flex-col gap-2 bg-black/90 p-3 rounded-bl-4">
                  <BaseRadioButton
                    checked={view() === View.Robot}
                    onClick={() => setView(View.Robot)}
                    label="Robot View"
                    name="view"
                  />
                  <BaseRadioButton
                    checked={view() === View.Camera}
                    onClick={() => setView(View.Camera)}
                    label="Camera View"
                    name="view"
                  />
                </div>
              </div>
              <div class="overflow-auto whitespace-pre-wrap break-all h-full w-full">
                <p>{JSON.stringify(currentFrame())}</p>
              </div>
            </Split>
          </Show>
        </div>
      </Split>
    </div>
  );
};

export default App;
