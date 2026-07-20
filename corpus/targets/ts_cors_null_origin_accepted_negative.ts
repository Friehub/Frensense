// SAFE: reject null origins explicitly
import cors from 'cors';

app.use(cors({
  origin: (origin, callback) => {
    // SAFE: reject null origin
    if (!origin || origin === 'null') {
      return callback(null, false);
    }
    if (ALLOWED_ORIGINS.includes(origin)) {
      callback(null, true);
    } else {
      callback(new Error('Not allowed by CORS'));
    }
  },
  credentials: true,
}));
