import { Component, For, Match, Show, Switch } from "solid-js";
import { BaseCheckBox } from "./Base/BaseCheckBox";
import { BaseInput } from "./Base/BaseInput";
import { BaseSlider } from "./Base/BaseSlider";

type allowed = string | number | boolean | null | allowed[] | jsonObject;
type jsonObject = { [key: string]: allowed };
type json = allowed | jsonObject;

interface SliderOptions {
  min: number;
  max: number;
  step: number;
}

type StructureObject = { [key: string]: structure };
type structure = SliderOptions | StructureObject | structure[] | undefined;

export const JSONEditor: Component<{
  name?: string;
  root?: boolean;
  data: json;
  structure: structure;
  path: (string | number)[];
  onChange: (path: (string | number)[], value: json) => void;
}> = (props) => {
  if (props.data === undefined || props.data === null) {
    return <h1>H</h1>;
  }

  return (
    <>
      {/* if it is a nested type (not fundamental type) */}
      <Show
        when={typeof props.data !== "object"}
        fallback={
          <>
            <p class="text-sm">{props.name}</p>
            <div class="ml-4 flex flex-col gap-2">
              <Show
                when={Array.isArray(props.data)}
                fallback={
                  <>
                    <For each={Object.keys(props.data as jsonObject)}>
                      {(key) => (
                        <JSONEditor
                          name={key}
                          data={(props.data as jsonObject)[key]}
                          structure={
                            (props.structure as StructureObject)?.[key]
                          }
                          path={[...props.path, key]}
                          onChange={props.onChange}
                        />
                      )}
                    </For>
                  </>
                }
              >
                <For each={props.data as json[]}>
                  {(item, i) => (
                    <JSONEditor
                      name={i().toString()}
                      data={item}
                      path={[...props.path, i()]}
                      onChange={props.onChange}
                      structure={
                        (props.structure as structure[] | undefined)?.[0]
                      }
                    />
                  )}
                </For>
              </Show>
            </div>
          </>
        }
      >
        {/* if it is a fundamental data type */}
        <div class="flex flex-col gap-2">
          <span class="text-sm w-8 text-ellipsis">{props.name}</span>
          <div class="flex gap-2 items-center">
            <Switch fallback={<p>unhandled type {typeof props.data}</p>}>
              <Match when={typeof props.data === "string"}>
                <BaseInput
                  class="!p-2.5 w-full"
                  label={props.name}
                  value={props.data as string}
                  onInput={(ev) =>
                    props.onChange(props.path, ev.currentTarget.value)
                  }
                />
              </Match>
              <Match when={typeof props.data === "number"}>
                {() => {
                  const structure = props.structure as SliderOptions;
                  return (
                    <BaseSlider
                      class="w-full"
                      min={structure.min}
                      max={structure.max}
                      step={structure.step}
                      value={props.data as number}
                      onInput={(ev) =>
                        props.onChange(
                          props.path,
                          ev.currentTarget.valueAsNumber
                        )
                      }
                    />
                  );
                }}
                <BaseInput
                  class="!p-2.5 w-16"
                  label={props.name}
                  type="number"
                  value={props.data as number}
                  onInput={(ev) =>
                    props.onChange(props.path, ev.currentTarget.valueAsNumber)
                  }
                />
              </Match>
              <Match when={typeof props.data === "boolean"}>
                <BaseCheckBox
                  class="!p-2.5"
                  value={props.data as number}
                  onInput={(ev) =>
                    props.onChange(props.path, ev.currentTarget.checked)
                  }
                />
              </Match>
            </Switch>
          </div>
        </div>
      </Show>
    </>
  );
};
