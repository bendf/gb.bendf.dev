import { test, expect } from "@playwright/test";

test("Emulator displays black screen on boot", async ({ page }) => {
  await page.goto("/");

  // TODO: Avoid using timeout. Give it a second to load
  await page.waitForTimeout(1000);

  const canvasData = await page.evaluate(() => {
    const screen = document.getElementById("squaregb-screen");
    if (screen instanceof HTMLCanvasElement) {
      const context = screen.getContext("2d")!;

      const imgData = context.getImageData(0, 0, 160, 144);
      return imgData.data;
    }
    throw "Missing #squaregb-screen";
  });

  const data = new Uint8Array(canvasData);

  const expectedData = new Uint8Array(160 * 144 * 4);
  for (let i = 3; i < expectedData.length; i += 4) {
    expectedData[i] = 0xff;
  }

  expect(data).toEqual(expectedData);
});

// Emulator displays black in to-right-hand corner
test.skip("Emulator test rom shows checkerboard", async ({ page }) => {
  await page.goto("/");

  const black = [0x00, 0x00, 0x00, 0x01];
  const white = [0xff, 0xff, 0xff, 0x01];

  page.waitForTimeout(1000);

  await page.getByText("Load Checkerboard ROM").click();
  await page.getByTestId("button-run-emulator").click();

  page.waitForTimeout(1000);

  const firstTileColor = await page.evaluate(() => {
    const screen = document.getElementById("squaregb-screen");

    if (screen instanceof HTMLCanvasElement) {
      const context = screen.getContext("2d")!;

      const imgData = context.getImageData(0, 0, 1, 1);
      return [
        imgData.data[0],
        imgData.data[1],
        imgData.data[2],
        imgData.data[3],
      ];
    }
  });

  expect(firstTileColor).toBe(black);

  const secondTileColor = await page.evaluate(() => {
    const screen = document.getElementById("squaregb-screen");

    if (screen instanceof HTMLCanvasElement) {
      const context = screen.getContext("2d")!;

      const imgData = context.getImageData(8, 0, 1, 1);
      return [
        imgData.data[0],
        imgData.data[1],
        imgData.data[2],
        imgData.data[3],
      ];
    }
  });

  expect(secondTileColor).toBe(white);
});
