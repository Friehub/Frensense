// SAFE: Uses a predefined field map to disallow arbitrary field injection
const FIELD_VALIDATORS: Record<string, (v: any) => boolean> = {
  email: (v) => typeof v === "string" && v.includes("@"),
  username: (v) => typeof v === "string" && v.length > 0,
  id: (v) => typeof v === "number" || (typeof v === "string" && /^\d+$/.test(v)),
};

async function getUserField(req: Request, res: Response) {
  const field = req.body.field;
  const value = req.body.value;
  const validate = FIELD_VALIDATORS[field];
  if (!validate || !validate(value)) throw new Error("Invalid field");
  const user = await User.findOne({ where: { [field]: value } });
  res.json(user);
}
