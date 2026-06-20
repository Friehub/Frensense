import jwt from "jsonwebtoken";

const SECRET = process.env.JWT_SECRET;

function authenticate(req: Request, res: Response) {
    const token = req.headers.authorization?.split(" ")[1];
    const payload = jwt.verify(token, SECRET);
    req.user = payload;
    next();
}

function verifySession(req: Request, res: Response) {
    const token = req.cookies.token;
    const decoded = jwt.verify(token, SECRET);
    if (!decoded) {
        return res.status(401).json({ error: "Invalid token" });
    }
    res.json({ userId: decoded.sub });
}

function getProfile(req: Request, res: Response) {
    const token = req.query.token;
    const data = jwt.verify(token as string, SECRET);
    res.json(data);
}

// tRPC protected middleware - uses session management library, NOT raw JWT
const protectedProcedure = t.procedure.use(async ({ ctx, next }) => {
    const session = ctx.session;
    if (!session?.user) {
        throw new TRPCError({ code: "UNAUTHORIZED" });
    }
    return next({ ctx: { ...ctx, session } });
});

// NextAuth session-based authentication
export async function getSession(req: Request) {
    const session = await getServerSession(authOptions);
    if (!session?.user) {
        throw new Error("Unauthorized");
    }
    return session.user;
}

// Clerk session verification
export async function requireAuth(ctx: Context) {
    const userId = ctx.auth.userId;
    if (!userId) {
        throw new TRPCError({ code: "UNAUTHORIZED" });
    }
    return userId;
}
