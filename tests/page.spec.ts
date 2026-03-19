import { test, expect } from "@playwright/test";

test("Has title", async ({ page }) => {
  await page.goto("/");

  // Expect a title "to contain" a substring.
  await expect(page).toHaveTitle(/Ben's GB emulator/);
});

test("Has header", async ({ page }) => {
  await page.goto("/");

  await expect(
    page.getByRole("heading", { name: "Ben's GB emulator" }),
  ).toBeVisible();
});

test("Renders gameboy-size canvas", async ({ page }) => {
  await page.goto("/");

  const canvas = page.locator("#squaregb-screen");
  await expect(canvas).toBeVisible();
  expect(await canvas.getAttribute("width")).toBe("160");
  expect(await canvas.getAttribute("height")).toBe("144");
});
