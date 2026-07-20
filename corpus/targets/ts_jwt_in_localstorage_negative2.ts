// SAFE: Token managed by the server via session cookie with strict flags
export function configureSession(app: express.Application): void {
  app.use(session({
    name: '__Host-sid',
    secret: process.env.SESSION_SECRET!,
    cookie: {
      httpOnly: true,
      secure: true,
      sameSite: 'strict',
      path: '/'
    },
    resave: false,
    saveUninitialized: false
  }));
}
