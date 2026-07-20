// SAFE: Uses mathjs expression parser instead of eval for safe math evaluation
import { evaluate } from "mathjs";

function processExpression(expr: string) {
  return evaluate(expr);
}
