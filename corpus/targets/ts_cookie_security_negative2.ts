// SAFE: Uses helmet's cookie security defaults via a global middleware
import helmet from "helmet";

app.use(helmet());

function handler(req: any, res: any) {
    res.cookie("session", "value", { httpOnly: true, secure: true, sameSite: "strict" });
}
