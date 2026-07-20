// SAFE: Limits the number of operations per batch request
import { ApolloServer } from 'apollo-server-express';

const MAX_OPERATIONS = 5;

function batchLimitMiddleware(req: Request, res: Response, next: NextFunction): void {
  if (Array.isArray(req.body) && req.body.length > MAX_OPERATIONS) {
    return res.status(400).json({ error: `Maximum ${MAX_OPERATIONS} operations per request` });
  }
  next();
}

app.post('/graphql', batchLimitMiddleware, server.getMiddleware());
