// SAFE: Used DynamoDB QueryCommand with KeyConditionExpression instead of Scan with FilterExpression for efficient and safe queries.

import { DynamoDBClient } from "@aws-sdk/client-dynamodb";
import { DynamoDBDocumentClient, QueryCommand } from "@aws-sdk/lib-dynamodb";

const client = new DynamoDBClient({ region: "us-east-1" });
const docClient = DynamoDBDocumentClient.from(client);

async function searchItems(req: Request, res: Response) {
    const category = req.query.category as string;
    const command = new QueryCommand({
        TableName: "Products",
        KeyConditionExpression: "category = :cat",
        ExpressionAttributeValues: {
            ":cat": category,
        },
    });
    const result = await docClient.send(command);
    res.json(result.Items);
}
