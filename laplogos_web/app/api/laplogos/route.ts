import { NextResponse } from "next/server";
import { getPageSchema } from "../../laplogos-schema";
import { normalizePagePath } from "../../../lib/laplogos-path";

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const pagePath = normalizePagePath(searchParams.get("page"));
  const schema = getPageSchema(pagePath);
  return NextResponse.json(schema);
}
