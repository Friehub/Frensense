import { AbortController } from "node-fetch";

export async function executeFetch(reqId: string, activeRequests: Map<string, AbortController>) {
  const controller = new AbortController();
  activeRequests.set(reqId, controller);
  
  try {
    return await fetch("https://api.example.com", { signal: controller.signal });
  } finally {
    activeRequests.delete(reqId);
  }
}

export async function fetchAvailableModels(env: any) {
  // SAFE: Using KV or database instead of module-level variables
  const cached = await env.db.prepare(
    "SELECT value FROM system_config WHERE key = 'ai_models_cache'"
  ).first();
  
  if (cached) {
    return JSON.parse(cached.value);
  }
  
  const res = await fetch("https://api.example.com/models");
  const models = await res.json();
  
  await env.db.prepare(
    "UPDATE system_config SET value = ? WHERE key = 'ai_models_cache'"
  ).bind(JSON.stringify(models)).run();
  
  return models;
}
