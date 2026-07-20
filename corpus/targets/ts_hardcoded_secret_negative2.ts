// SAFE: Loads secrets from a config service at runtime instead of hardcoding
import { config } from "./config";

function getApiKey() {
  return config.get("apiKey");
}
