export function normalizePagePath(input?: string | null) {
  if (!input) return "/";
  if (input.startsWith("/")) return input;
  return `/${input}`;
}
