// SAFE: Uses a specific known origin instead of wildcard when credentials are needed
export function makeCors(): Record<string, string> {
  return {
    "Access-Control-Allow-Origin": "https://friehub.com",
    "Access-Control-Allow-Methods": "GET, POST, OPTIONS",
    "Access-Control-Allow-Headers": "Content-Type, Authorization",
    "Access-Control-Allow-Credentials": "true",
  };
}
