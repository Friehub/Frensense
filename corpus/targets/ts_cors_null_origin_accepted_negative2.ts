// SAFE alternative: specific origins only, no null or undefined
import cors from 'cors';

const SPECIFIC_ORIGINS = ['https://app.example.com', 'https://admin.example.com'];

app.use(cors({
  origin: SPECIFIC_ORIGINS,
  credentials: true,
}));
