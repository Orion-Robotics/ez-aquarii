import { children, Component, onCleanup, onMount, splitProps } from "solid-js";
import split from "split.js";

export const Split: Component<
  {
    gutterClass?: string;
  } & Split.Options
> = (props) => {
  const c = children(() => props.children);

  onMount(() => {
    const [_, rest] = splitProps(props, ["children"]);
    const inst = split(c() as HTMLElement[], {
      gutter: (index, direction) => {
        const gutter = document.createElement("div");
        gutter.className = props.gutterClass || `gutter gutter-${direction}`;
        return gutter;
      },
      ...rest,
    });
    onCleanup(() => inst.destroy());
  });

  return c;
};
