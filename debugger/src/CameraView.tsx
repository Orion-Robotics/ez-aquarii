import { Component } from "solid-js";
import { css } from "vite-plugin-inline-css-modules";

const styles = css`
  .slider {
    @apply w-40 md:w-80;
  }
`;

const pages = ["Camera Settings", "Ball", "Yellow Goal", "Blue Goal"];

export const CameraView: Component<{
  host: string;
}> = (props) => {
  return (
    <div class="h-full w-full flex justify-center">
      <img
        class="object-contain h-full max-w-full max-h-full"
        src={`http://${props.host}/camera`}
      />
    </div>
  );
};
