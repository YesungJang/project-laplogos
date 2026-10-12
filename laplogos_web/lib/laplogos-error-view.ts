/**
 * Shared presentation pieces for the error display.
 *
 * The CSR component (`components/ErrorView.tsx`) and the SSR fallback
 * (`lib/laplogos-ssr.ts#renderErrorHtml`) render the same alert, so both read
 * the text, the classes and the message normalization from here instead of
 * keeping two copies in sync by hand.
 */

export const ERROR_VIEW_TITLE = "Laplogos Error";

/** Shown when the message is not usable text (blank, or not a string). */
export const ERROR_VIEW_FALLBACK_MESSAGE = "不明なエラーが発生しました。";

/** 復旧のための具体的な手順（1 文）。 */
export const ERROR_VIEW_RECOVERY_TEXT =
  "ページを再読み込みするか、schema を修正して再試行してください。";

export const ERROR_VIEW_DEBUG_TEXT = "Debug View を確認してください。";

export const ERROR_VIEW_CONTAINER_CLASS =
  "rounded-md border border-rose-200 bg-rose-50 p-4 text-rose-900";
export const ERROR_VIEW_TITLE_CLASS = "text-base font-semibold";
export const ERROR_VIEW_MESSAGE_CLASS = "mt-2 text-sm";
export const ERROR_VIEW_HINT_CLASS = "mt-2 text-xs text-rose-700";

/**
 * Normalizes the message for display.
 *
 * Callers type the message as `string`, but it reaches the error display from
 * runtime failure paths, so a non-string or blank value falls back to a stable
 * sentence instead of rendering `undefined`, `[object Object]` or an alert
 * that announces nothing.
 */
export function toErrorViewMessage(message: unknown): string {
  if (typeof message !== "string" || message.trim().length === 0) {
    return ERROR_VIEW_FALLBACK_MESSAGE;
  }
  return message;
}
