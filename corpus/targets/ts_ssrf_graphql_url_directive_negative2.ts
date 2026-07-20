// SAFE: Uses pre-defined local schema files instead of remote introspection; no URL-based schema stitching
import { stitchSchemas } from "@graphql-tools/stitch";
import { loadSchemaSync } from "@graphql-tools/load";
import { GraphQLFileLoader } from "@graphql-tools/graphql-file-loader";

export function buildStitchedSchema() {
    const localSchemas = [
        loadSchemaSync("./schemas/users.graphql", { loaders: [new GraphQLFileLoader()] }),
        loadSchemaSync("./schemas/orders.graphql", { loaders: [new GraphQLFileLoader()] }),
    ];
    return stitchSchemas({ subschemas: localSchemas.map(s => ({ schema: s })) });
}
