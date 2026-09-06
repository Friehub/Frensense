// SAFE: Used a query builder with predefined filter templates; never accepts raw OData strings from user input.

const FILTER_TEMPLATES: Record<string, (val: string) => string> = {
    "name": (v) => `contains(name, '${v.replace(/'/g, "''")}')`,
    "price_gt": (v) => `price gt ${parseFloat(v)}`,
    "price_lt": (v) => `price lt ${parseFloat(v)}`,
    "category": (v) => `category eq '${v.replace(/'/g, "''")}'`,
    "status": (v) => `status eq '${v.replace(/'/g, "''")}'`,
};

async function getProducts(req: Request, res: Response) {
    const filterName = req.query.filter as string;
    const filterValue = req.query.value as string;
    const builder = FILTER_TEMPLATES[filterName];
    if (!builder) throw new Error("Unknown filter");
    const filter = builder(filterValue);
    const result = await client.get("Products", { filter });
    res.json(result);
}

async function searchOrders(req: Request, res: Response) {
    const result = await client.get("Orders", {
        filter: "status eq 'active'",
        orderby: "createdAt desc",
    });
    res.json(result);
}
