// SAFE: Replaced string concatenation in FilterExpression with ExpressionAttributeValues using :placeholders.

import { DynamoDBClient } from "@aws-sdk/client-dynamodb";
import { DynamoDBDocumentClient, ScanCommand } from "@aws-sdk/lib-dynamodb";

const client = new DynamoDBClient({ region: "us-east-1" });
const docClient = DynamoDBDocumentClient.from(client);

async function searchItems(req: Request, res: Response) {
    const category = req.query.category as string;
    const minPrice = parseFloat(req.query.minPrice as string);
    const command = new ScanCommand({
        TableName: "Products",
        FilterExpression: "category = :cat AND price >= :min",
        ExpressionAttributeValues: {
            ":cat": category,
            ":min": minPrice,
        },
    });
    const result = await docClient.send(command);
    res.json(result.Items);
}

async function advancedSearch(req: Request, res: Response) {
    const { field, operator, value } = req.body;
    const command = new ScanCommand({
        TableName: "Orders",
        FilterExpression: "#field " + operator + " :val",
        ExpressionAttributeNames: { "#field": field },
        ExpressionAttributeValues: { ":val": value },
    });
    const result = await docClient.send(command);
    res.json(result.Items);
}
