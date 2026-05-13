import { test, expect } from "@playwright/test";

test("Runs basic wasm code", async ({ page }) => {
  await page.goto("/");

  await expect(page.getByTestId("run-wasm-test")).toBeVisible();

  await expect(page.getByTestId("wasm-console")).toHaveText("");

  await page.getByTestId("run-wasm-test").click();

  await expect(page.getByTestId("wasm-console")).toHaveText("Hello, World!");

  await page.screenshot({ path: "hello_world.png" });
});
