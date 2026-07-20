// [frensense]
// observation: CORS configuration accepts 'null' Origin, allowing sandboxed iframes or data: URIs to make authenticated cross-origin requests.
// impact: Any sandboxed iframe (e.g., ads, embeds, file://) can set Origin: null and bypass CORS restrictions. This enables data theft from authenticated users who visit attacker-controlled pages.
// improvement: Skip null origins in the CORS allowlist. Only accept specific origins that match your application's real domains.

import cors from 'cors';

app.use(cors({
  // VULNERABLE: accepts null origin
  origin: (origin, callback) => {
    if (!origin || ALLOWED_ORIGINS.includes(origin)) {
      callback(null, true);
    } else {
      callback(new Error('Not allowed by CORS'));
    }
  },
  credentials: true,
}));
