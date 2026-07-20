// SAFE: Remote schema URLs are validated against an allowlist before introspection
import { stitchSchemas } from "@graphql-tools/stitch";
import { introspectFromUrl } from "@graphql-tools/wrap";

const ALLOWED_SCHEMA_ENDPOINTS = new Set([
    "https://graphql.trusted.com/graphql",
    "https://api.trusted.com/graphql",
]);

export async function buildStitchedSchema(urls: string[]) {
    const filtered = urls.filter(u => ALLOWED_SCHEMA_ENDPOINTS.has(u));
    const subschemas = await Promise.all(
        filtered.map(async (url) => ({
            schema: await introspectFromUrl(url),
        }))
    );
    return stitchSchemas({ subschemas });
}

export async function addRemoteSchema(url: string) {
    if (!ALLOWED_SCHEMA_ENDPOINTS.has(url)) {
        throw new Error("Remote schema URL not allowed");
    }
    return introspectFromUrl(url);
}
