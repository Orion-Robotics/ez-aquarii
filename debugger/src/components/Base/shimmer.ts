import { children, Component, createEffect, createSignal } from "solid-js";
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

const shimmerStyles = css`
  .overlay {
    position: absolute;
    width: 100%;
    height: 100%;
    top: 0;
    left: 0;
    pointer-events: none;
  }
`;

export const Shimmer: Component<{
  opacity?: number;
  color?: string;
}> = (props) => {
  const color = hexToRgb(props.color || "#ffffff");
  const [x, setX] = createSignal(0);
  const [y, setY] = createSignal(0);
  const [hover, setHover] = createSignal(false);
  const c = children(() => props.children);
  let overlayEl: HTMLElement | undefined = undefined;

  createEffect(() => {
    const el = c() as HTMLElement;
    el.style.position = "relative";
    const overlay = document.createElement("div");
    overlay.className = shimmerStyles.overlay;
    el.addEventListener("mousemove", ({ clientX, clientY }) => {
      const rect = overlay.getBoundingClientRect();
      setX(clientX - rect.left);
      setY(clientY - rect.top);
    });
    el.addEventListener("mouseleave", () => setHover(true));
    el.addEventListener("mouseenter", () => setHover(false));
    el.appendChild(overlay);
    overlayEl = overlay;
  });

  createEffect(() => {
    if (hover) {
      overlayEl.style.cssText = `background: radial-gradient(circle at ${x()}px ${y()}px, rgba(255, 255, 255, 0.2) 0, transparent 100%);`;
    } else {
      overlayEl.style.cssText = "";
    }
  });

  return c();
};
