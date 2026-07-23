async function getUserField(req: Request, res: Response) {
    const field = req.body.field;
    const value = req.body.value;
    const user = await User.findOne({ where: { [field]: value } });
    res.json(user);
}
