// SAFE: Built OData filter from parts using allowed fields only, preventing arbitrary expression injection.

const ALLOWED_FILTER_FIELDS = new Set(["name", "price", "category", "status", "createdAt"]);
const ALLOWED_OPERATORS = new Set(["eq", "ne", "gt", "gte", "lt", "lte", "contains"]);

function safeODataFilter(filters: Array<{ field: string; op: string; value: string }>): string {
    return filters
        .filter(f => ALLOWED_FILTER_FIELDS.has(f.field) && ALLOWED_OPERATORS.has(f.op))
        .map(f => `${f.field} ${f.op} '${f.value.replace(/'/g, "''")}'`)
        .join(" and ");
}

async function getProducts(req: Request, res: Response) {
    const filterExpr = safeODataFilter([{
        field: req.query.field as string || "name",
        op: "contains",
        value: req.query.q as string || "",
    }]);
    const result = await client.get("Products", { filter: filterExpr });
    res.json(result);
}

async function searchOrders(req: Request, res: Response) {
    const filters = req.body.filters || [];
    const filter = safeODataFilter(filters);
    const result = await client.get("Orders", { filter });
    res.json(result);
}
