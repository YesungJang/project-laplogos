import assert from "node:assert/strict";
import test from "node:test";
import { getPageSchema } from "../app/laplogos-schema";
import { normalizePagePath } from "../lib/laplogos-path";
import { escapeJsonForScript, renderLaplogosHtml } from "../lib/laplogos-ssr";

test("ssr renders with data ids and escapes bootstrap json", async () => {
  const pagePath = normalizePagePath("/");
  const page = getPageSchema(pagePath);
  const state = { note: "<tag>" };

  const html = await renderLaplogosHtml(page, state);
  assert.ok(html.includes("data-laplogos-id"));

  const bootstrap = escapeJsonForScript({ page, state });
  assert.ok(bootstrap.includes("\\u003c"));
  assert.ok(!bootstrap.includes("<"));
});

test("page path normalization keeps url paths intact", () => {
  assert.equal(normalizePagePath("/docs/intro"), "/docs/intro");
  assert.equal(normalizePagePath("docs/intro"), "/docs/intro");
});
