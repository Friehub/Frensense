const ALLOWED_FIELDS = ["email", "username", "id"];

async function getUserField(req: Request, res: Response) {
    const field = req.body.field;
    const value = req.body.value;
    if (!ALLOWED_FIELDS.includes(field)) throw new Error("Invalid field");
    const user = await User.findOne({ where: { [field]: value } });
    res.json(user);
}
