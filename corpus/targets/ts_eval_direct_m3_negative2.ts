// SAFE: Uses mathjs expression parser instead of eval
import { evaluate } from "mathjs";
function handlerA(req: Request, res: Response) {
  const result = evaluate(req.body.expression);
  res.json({ result });
}
function handlerB(req: Request, res: Response) {
  const result = evaluate(req.query.code);
  res.json({ result });
}
