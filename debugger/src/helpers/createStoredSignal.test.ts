import { describe, expect, test } from "vitest";
import { createStoredSignal } from "./createStoredSignal";

describe("createStoredSignal", () => {
  beforeEach(() => localStorage.clear())
  test("should be set to initial on startup", () => {
    const [value, setValue] = createStoredSignal(1, "test");
    expect(value()).toBe(1);
  });
  test("should change localstorage on change", () => {
    const [value, setValue] = createStoredSignal(1, "test");
    setValue(2);
    expect(localStorage.getItem("test")).toBe("2");
  });
});
