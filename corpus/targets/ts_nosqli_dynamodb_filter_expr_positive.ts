// [frensense]
// observation: User-controlled values are concatenated directly into DynamoDB FilterExpression strings, allowing NoSQL injection through expression attribute manipulation.
// impact: An attacker can craft input that breaks the expression syntax, bypassing filters or extracting data from unrelated attributes.
// improvement: Use ExpressionAttributeValues with placeholders (:val) instead of string concatenation in FilterExpression.

import { DynamoDBClient } from "@aws-sdk/client-dynamodb";
import { DynamoDBDocumentClient, ScanCommand } from "@aws-sdk/lib-dynamodb";

const client = new DynamoDBClient({ region: "us-east-1" });
const docClient = DynamoDBDocumentClient.from(client);

async function searchItems(req: Request, res: Response) {
    const category = req.query.category as string;
    const minPrice = req.query.minPrice as string;
    const command = new ScanCommand({
        TableName: "Products",
        FilterExpression: "category = '" + category + "' AND price >= " + minPrice,
    });
    const result = await docClient.send(command);
    res.json(result.Items);
}

async function advancedSearch(req: Request, res: Response) {
    const { field, operator, value } = req.body;
    const command = new ScanCommand({
        TableName: "Orders",
        FilterExpression: field + " " + operator + " " + value,
    });
    const result = await docClient.send(command);
    res.json(result.Items);
}
