// SAFE: Uses the cors npm package with explicit origin configuration
import cors from "cors";

const corsOptions = {
  origin: ["https://friehub.com", "https://taas.friehub.com"],
  credentials: true,
};

app.use(cors(corsOptions));
