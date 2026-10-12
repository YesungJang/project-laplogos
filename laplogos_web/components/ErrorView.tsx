'use client';

import {
  ERROR_VIEW_CONTAINER_CLASS,
  ERROR_VIEW_DEBUG_TEXT,
  ERROR_VIEW_HINT_CLASS,
  ERROR_VIEW_MESSAGE_CLASS,
  ERROR_VIEW_RECOVERY_TEXT,
  ERROR_VIEW_TITLE,
  ERROR_VIEW_TITLE_CLASS,
  toErrorViewMessage,
} from "../lib/laplogos-error-view";

type Props = {
  message: string;
};

export function ErrorView({ message }: Props) {
  // The message always renders as plain text; unusable values fall back.
  const text = toErrorViewMessage(message);

  return (
    <div
      role="alert"
      aria-live="assertive"
      aria-atomic="true"
      className={ERROR_VIEW_CONTAINER_CLASS}
    >
      <h3 className={ERROR_VIEW_TITLE_CLASS}>{ERROR_VIEW_TITLE}</h3>
      <p className={ERROR_VIEW_MESSAGE_CLASS}>{text}</p>
      <p className={ERROR_VIEW_HINT_CLASS}>{ERROR_VIEW_RECOVERY_TEXT}</p>
      <p className={ERROR_VIEW_HINT_CLASS}>{ERROR_VIEW_DEBUG_TEXT}</p>
    </div>
  );
}
