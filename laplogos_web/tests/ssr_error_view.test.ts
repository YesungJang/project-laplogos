import assert from "node:assert/strict";
import test from "node:test";
import { renderErrorHtml } from "../lib/laplogos-ssr";

// Mirrors the recovery line rendered by lib/laplogos-ssr.ts and
// components/ErrorView.tsx.
const RECOVERY_INSTRUCTION =
  "ページを再読み込みし、解決しない場合は Debug View を確認してください。";

test("SSR error view announces the error and tells the user what to do", () => {
  const html = renderErrorHtml("SSR failed");
  assert.ok(html.includes('role="alert"'), "error must be announced as an alert");
  assert.ok(html.includes("Laplogos Error"));
  assert.ok(html.includes(">SSR failed<"), "message must be rendered as text");
  assert.ok(html.includes(RECOVERY_INSTRUCTION), "recovery instruction must be shown");
});

test("SSR error view escapes message", () => {
  const html = renderErrorHtml("<tag>");
  assert.ok(html.includes("Laplogos Error"));
  assert.ok(html.includes("&lt;tag&gt;"));
  assert.ok(!html.includes("<tag>"));
});

test("SSR error view escapes markup, ampersands and quotes in hostile input", () => {
  const html = renderErrorHtml('<script>alert("x") && 1</script>');

  assert.ok(html.includes("&lt;script&gt;"));
  assert.ok(html.includes("&lt;/script&gt;"));
  assert.ok(html.includes("&quot;x&quot;"));
  assert.ok(html.includes("&amp;&amp;"));

  assert.ok(!html.includes("<script"), "no raw script tag may survive");
  assert.ok(!html.includes("</script>"));
  assert.ok(!html.includes('alert("x")'), "quotes must stay escaped");

  // The surrounding chrome is unaffected by a hostile message.
  assert.ok(html.includes('role="alert"'));
  assert.ok(html.includes(RECOVERY_INSTRUCTION));
});

test("SSR error view stays usable for an empty message", () => {
  const html = renderErrorHtml("");

  assert.ok(html.includes('role="alert"'));
  assert.ok(html.includes("Laplogos Error"));
  assert.ok(html.includes('<p class="mt-2 text-sm"></p>'), "message element is kept but empty");
  assert.ok(html.includes(RECOVERY_INSTRUCTION));
});

test("SSR error view keeps the message verbatim", () => {
  const padded = "  spaced  ";
  const html = renderErrorHtml(padded);
  assert.ok(html.includes(`>${padded}<`), "message must not be trimmed or reformatted");

  const multiline = "line one\nline two";
  assert.ok(renderErrorHtml(multiline).includes(`>${multiline}<`));
});

test("SSR error view handles long and multi-byte messages", () => {
  const long = "e".repeat(10_000);
  const longHtml = renderErrorHtml(long);
  assert.ok(longHtml.includes(long), "a long message must not be truncated");
  assert.ok(longHtml.includes(RECOVERY_INSTRUCTION));

  const japanese = "スキーマの描画に失敗しました（ノード数の上限を超過）。";
  const japaneseHtml = renderErrorHtml(japanese);
  assert.ok(japaneseHtml.includes(`>${japanese}<`));
  assert.ok(japaneseHtml.includes('role="alert"'));
});
