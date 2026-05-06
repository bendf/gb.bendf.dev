import { test, expect, Page } from "@playwright/test";

function getScreenPixel(
  page: Page,
  x: number,
  y: number,
): Promise<[number, number, number, number]> {
  return page.evaluate(
    ({ x, y }) => {
      const screen = document.getElementById("squaregb-screen");

      if (screen instanceof HTMLCanvasElement) {
        const context = screen.getContext("2d")!;

        const imgData = context.getImageData(x, y, 1, 1);
        return [
          imgData.data[0],
          imgData.data[1],
          imgData.data[2],
          imgData.data[3],
        ];
      }
      throw new Error("Screen not found");
    },
    { x, y },
  );
}

test("Emulator displays black screen on boot", async ({ page }) => {
  await page.goto("/");

  await page.waitForTimeout(1000);

  await page.screenshot({ path: "boot_black_screen.png" });

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

test("Emulator test rom shows checkerboard", async ({ page }) => {
  await page.goto("/");

  const black = [0x00, 0x00, 0x00, 0xff];
  const white = [0xff, 0xff, 0xff, 0xff];

  await page.getByText("Load Checkerboard ROM").click();
  await page.getByTestId("input-lcdc.5").setChecked(false);
  await page.getByText("Render screen").click();

  await page.waitForTimeout(1000);

  const firstTileColor = await getScreenPixel(page, 0, 0);
  expect(firstTileColor).toStrictEqual(black);

  const secondTileColor = await getScreenPixel(page, 8, 0);
  expect(secondTileColor).toStrictEqual(white);
});

test("Emulator test rom shows scrolling background", async ({ page }) => {
  await page.goto("/");

  const black = [0x00, 0x00, 0x00, 0xff];
  const white = [0xff, 0xff, 0xff, 0xff];

  await page.getByText("Load Checkerboard ROM").click();
  await page.getByTestId("input-lcdc.5").setChecked(false);
  await page.getByText("Render screen").click();

  await page.waitForTimeout(1000);

  let firstTileColor = await getScreenPixel(page, 0, 0);
  expect(firstTileColor).toStrictEqual(black);

  await page.getByTestId("input-scx").fill("8");
  await page.getByText("Render screen").click();
  await page.waitForTimeout(1000);

  firstTileColor = await getScreenPixel(page, 0, 0);
  expect(firstTileColor).toStrictEqual(white);

  await page.getByTestId("input-scy").fill("8");
  await page.getByText("Render screen").click();
  await page.waitForTimeout(1000);

  firstTileColor = await getScreenPixel(page, 0, 0);

  expect(firstTileColor).toStrictEqual(black);
});

test("Emulator test rom shows white window on black background", async ({
  page,
}) => {
  await page.goto("/");

  const black = [0x00, 0x00, 0x00, 0xff];
  const white = [0xff, 0xff, 0xff, 0xff];

  await page.getByText("Load Window ROM").click();
  await page.getByTestId("input-lcdc.5").setChecked(true);

  await page.getByTestId("input-wx").fill("0");
  await page.getByTestId("input-wy").fill("77");
  await page.getByText("Render screen").click();

  await page.waitForTimeout(1000);

  let topLeftColor = await getScreenPixel(page, 0, 0);
  expect(topLeftColor).toStrictEqual(black);

  let botRightColor = await getScreenPixel(page, 159, 143);
  expect(botRightColor).toStrictEqual(white);
});

test("Emulator test rom shows white sprite centered on black background", async ({
  page,
}) => {
  await page.goto("/");

  const black = [0x00, 0x00, 0x00, 0xff];
  const white = [0xff, 0xff, 0xff, 0xff];

  await page.getByText("Load Sprite ROM").click();
  // Disable Window
  await page.getByTestId("input-lcdc.5").setChecked(false);

  await page.getByText("Render screen").click();

  await page.waitForTimeout(1000);

  let topLeftColor = await getScreenPixel(page, 0, 0);
  expect(topLeftColor).toStrictEqual(black);

  let centerColor = await getScreenPixel(page, 80, 72);
  expect(centerColor).toStrictEqual(white);

  let botRightColor = await getScreenPixel(page, 159, 143);
  expect(botRightColor).toStrictEqual(black);
});
