import { Component, createSignal, Match, Show, Switch } from "solid-js";
import ArrowLeft from "~icons/mdi/arrow-left";
import ArrowRight from "~icons/mdi/arrow-right";
import { ShimmerButton } from "./Base/BaseButton";
import { BaseInput } from "./Base/BaseInput";
import { BaseRadioButton } from "./Base/BaseRadioButton";
import { BaseSlider } from "./Base/BaseSlider";
import { Label } from "./Base/Label";
import { Split } from "./Base/Split";
import { CameraView } from "./CameraView";
import {
  DataObject,
  DataSource,
  ServerSource,
  TextSource,
} from "./data_sources";
import { createStoredSignal } from "./Helpers/createStoredSignal";
import { RobotView } from "./RobotView";

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
  const [cameraAddress, setCameraAddress] = createStoredSignal(
    "0.0.0.0:7272",
    "cameraAddress"
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

  let source: DataSource | undefined = undefined;

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
      <div class="w-72 bg-dark-600 h-full overflow-auto">
        <div class="px-4 py-4 flex flex-col gap-4">
          <a class="hover:underline underline-offset-4">Nebula</a>
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
              <Label>Camera Address</Label>
              <BaseInput
                label="0.0.0.0:7273"
                onInput={(ev) => setCameraAddress(ev.currentTarget.value)}
                value={cameraAddress()}
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
        </div>
      </div>
      <Show
        when={currentFrame()}
        fallback={<div class="flex-1 p-3">Press start to begin monitoring</div>}
      >
        <div class="flex-1 flex-col flex">
          <div class="bg-dark-300 p-2 flex gap-2">
            <ShimmerButton onClick={() => source?.next()}>
              <ArrowLeft />
            </ShimmerButton>
            <ShimmerButton onClick={() => source?.back()}>
              <ArrowRight />
            </ShimmerButton>
            <BaseSlider
              class="w-full"
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
            gutterClass="bg-dark-800 h-1 cursor-ns-resize active:bg-blue-500 transition duration-200"
          >
            <div class="relative">
              <Switch>
                <Match when={view() === View.Robot}>
                  <RobotView frame={currentFrame()!} />
                </Match>
                <Match when={view() === View.Camera}>
                  <CameraView host={cameraAddress()} />
                </Match>
              </Switch>
              <div class="absolute top-3 right-3 flex flex-col gap-2">
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
        </div>
      </Show>
    </div>
  );
};

export default App;
