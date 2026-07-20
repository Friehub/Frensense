// [frensense]
// observation: A GraphQL @external directive or schema stitching resolver fetches a user-provided URL to retrieve the remote schema, enabling SSRF via GraphQL.
// impact: An attacker can make the GraphQL server fetch arbitrary URLs, including internal services and cloud metadata, when the schema is being composed.
// improvement: Validate the URL against an allowlist before fetching the remote schema; disable URL-based schema stitching.

import { stitchSchemas } from "@graphql-tools/stitch";
import { introspectFromUrl } from "@graphql-tools/wrap";

export async function buildStitchedSchema(urls: string[]) {
    const subschemas = await Promise.all(
        urls.map(async (url) => ({
            schema: await introspectFromUrl(url),
        }))
    );
    return stitchSchemas({ subschemas });
}

export async function addRemoteSchema(url: string) {
    return introspectFromUrl(url);
}
