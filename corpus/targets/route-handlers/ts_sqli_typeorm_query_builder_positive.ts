async function searchUsers(req: Request, res: Response) {
    const name = req.query.name;
    const users = await AppDataSource.getRepository(User)
        .createQueryBuilder("user")
        .where(`user.name = '${name}'`)
        .getMany();
    res.json(users);
}
