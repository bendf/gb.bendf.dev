import { test, expect } from "@playwright/test";

test("Emulator displays logo on boot", async ({ page }) => {
  await page.goto("/");

  // Give it a second to load
  page.waitForTimeout(1000);
  const topLeft = await page.evaluate(() => {
    const screen = document.getElementById("#squaregb-screen");

    if (screen instanceof HTMLCanvasElement) {
      const context = screen.getContext("2d")!;

      const imgData = context.getImageData(0, 0, 1, 1);
      return imgData.data[0];
    }
  });

  expect(topLeft).toBe(100);
});
