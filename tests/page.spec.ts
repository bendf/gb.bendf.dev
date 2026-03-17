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

test("Renders a canvas", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByTestId("gb-screen")).toBeVisible();
});
