import { createComputed, createSignal } from "solid-js";

export function createStoredSignal<T>(initial: T | undefined, key: string) {
  const itemInStorage = localStorage.getItem(key);
  const signal = createSignal<T>(
    itemInStorage ? JSON.parse(itemInStorage) : initial
  );
  const [value, setValue] = signal;

  createComputed(() => {
    const v = value();
    localStorage.setItem(key, JSON.stringify(v));
  });

  window.addEventListener("storage", (ev) => {
    if (ev.key === key) {
      setValue(JSON.parse(ev.newValue));
    }
  });

  return signal;
}
