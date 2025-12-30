import { expect, test } from "@playwright/test";

test("focus guard keeps input value and focus", async ({ page }) => {
  await page.goto("/e2e/focus");
  const input = page.locator('input[data-laplogos-id="focus-input"]');
  await input.click();
  await input.fill("hello");
  await expect(input).toBeFocused();
  await expect(input).toHaveValue("hello");
});

test("reorder keeps input DOM and value", async ({ page }) => {
  await page.goto("/e2e/reorder-a");
  const inputA = page.locator('input[data-laplogos-id="row:a"]');
  await inputA.fill("typed");
  await expect(inputA).toHaveValue("typed");

  await page.locator('[data-laplogos-id="swap"]').click();
  await page.waitForURL("**/e2e/reorder-b");

  const inputAAfter = page.locator('input[data-laplogos-id="row:a"]');
  await expect(inputAAfter).toHaveValue("typed");

  const firstId = await page
    .locator('input[data-laplogos-id]')
    .first()
    .getAttribute("data-laplogos-id");
  expect(firstId).toBe("row:c");
});

test("hydration fallback replaces root on mismatch", async ({ page }) => {
  await page.goto("/e2e/focus?laplogosHydrationFail=1");
  await expect(page.locator('[data-laplogos-root="1"]')).toBeVisible();
  await expect(page.locator('input[data-laplogos-id="focus-input"]')).toBeVisible();
});
