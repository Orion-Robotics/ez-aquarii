import {
  children,
  Component,
  createEffect,
  createSignal,
  mergeProps,
} from "solid-js";
import { css } from "vite-plugin-inline-css-modules";

const hexToRgb = (hex: string) =>
  hex
    .replace(
      /^#?([a-f\d])([a-f\d])([a-f\d])$/i,
      (m, r, g, b) => "#" + r + r + g + g + b + b
    )
    .substring(1)
    .match(/.{2}/g)!
    .map((x) => parseInt(x, 16)) as [number, number, number];

export const Shimmer: Component<{
  opacity?: number;
  color?: string;
  waveTimeMs?: number;
}> = (_props) => {
  const props = mergeProps(
    {
      opacity: 0.5,
      color: "#ffffff",
      waveTimeMs: 200,
    },
    _props
  );
  const color = hexToRgb(props.color || "#ffffff");
  const [x, setX] = createSignal(0);
  const [y, setY] = createSignal(0);
  const [hover, setHover] = createSignal(false);
  const [downProgress, setDownProgress] = createSignal(0);
  const c = children(() => props.children);
  let overlayEl: HTMLElement | undefined = undefined;

  const styles = css`
    .container {
      position: absolute;
      width: 100%;
      height: 100%;
      top: 0;
      left: 0;
      pointer-events: none;
    }
  `;

  let lastFrame = 0;
  let backward = false;
  const downAnimationFunc = () => {
    const elapsed = Date.now() - lastFrame;
    const progress = Math.min(1, elapsed / props.waveTimeMs!);
    setDownProgress(backward ? 1 - progress : progress);

    if (progress < 1) {
      requestAnimationFrame(() => downAnimationFunc());
    } else if (!backward) {
      lastFrame = Date.now();
      backward = true;
      requestAnimationFrame(() => downAnimationFunc());
    } else {
      lastFrame = 0;
      setDownProgress(0);
    }
  };

  createEffect(() => {
    const el = c() as HTMLElement;
    el.style.position = "relative";
    const overlay = document.createElement("div");
    overlay.className = styles.container;
    el.addEventListener("mousemove", ({ clientX, clientY }) => {
      const rect = overlay.getBoundingClientRect();
      setX(clientX - rect.left);
      setY(clientY - rect.top);
    });
    el.addEventListener("mouseleave", () => setHover(false));
    el.addEventListener("mouseenter", () => setHover(true));
    el.addEventListener("mousedown", () => {
      lastFrame = Date.now();
      setDownProgress(0);
      backward = false;
      requestAnimationFrame(() => downAnimationFunc());
    });
    el.appendChild(overlay);
    overlayEl = overlay;
  });

  createEffect(() => {
    if (hover()) {
      overlayEl!.style.cssText = `background: radial-gradient(circle at ${x()}px ${y()}px, rgba(
        ${color[0]}, ${color[1]}, ${color[2]}, ${
        props.opacity - props.opacity * downProgress()
      }) ${backward ? 0 : downProgress() * 100}%, transparent 100%);`;
    } else {
      overlayEl!.style.cssText = "";
    }
  });

  return c();
};
