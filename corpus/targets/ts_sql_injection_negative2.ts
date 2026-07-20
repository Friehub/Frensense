// SAFE: Uses an ORM query builder instead of raw SQL for all database operations
async function searchUsers(req: Request, res: Response) {
  const name = req.query.name;
  const results = await User.findAll({ where: { name } });
  res.json(results);
}

async function getOrder(req: Request, res: Response) {
  const orderId = req.params.id;
  const order = await Order.findByPk(orderId);
  res.json(order);
}

async function filterProducts(req: Request, res: Response) {
  const category = req.body.category;
  const products = await Product.findAll({ where: { category, active: true } });
  res.json(products);
}
