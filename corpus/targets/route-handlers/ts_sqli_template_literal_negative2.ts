// SAFE: Used Prisma ORM typed queries which automatically parameterize inputs and prevent injection.

import { PrismaClient } from "@prisma/client";

const prisma = new PrismaClient();

async function getUserProfile(req: Request, res: Response) {
    const userId = req.params.id;
    const user = await prisma.user.findUnique({ where: { id: userId } });
    res.json(user);
}

async function searchProducts(req: Request, res: Response) {
    const term = req.query.q;
    const category = req.query.cat;
    const products = await prisma.product.findMany({
        where: {
            name: { contains: term },
            category: category,
        },
    });
    res.json(products);
}
