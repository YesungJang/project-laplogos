import assert from "node:assert/strict";
import test from "node:test";
import { renderErrorHtml } from "../lib/laplogos-ssr";

test("SSR error view escapes message", () => {
  const html = renderErrorHtml("<tag>");
  assert.ok(html.includes("Laplogos Error"));
  assert.ok(html.includes("&lt;tag&gt;"));
  assert.ok(!html.includes("<tag>"));
});
