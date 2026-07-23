import { AbortController } from "node-fetch";

// VULNERABILITY: Module-level state in serverless environment
// This map will be shared across requests and users, causing memory leaks
// and potentially returning wrong data to the wrong user if reused.
const activeRequests = new Map<string, AbortController>();
let _cachedModels: any[] | null = null;
let _cacheTime = 0;

export async function executeFetch(reqId: string) {
  const controller = new AbortController();
  activeRequests.set(reqId, controller);
  
  try {
    return await fetch("https://api.example.com", { signal: controller.signal });
  } finally {
    activeRequests.delete(reqId);
  }
}

export async function fetchAvailableModels() {
  if (_cachedModels && Date.now() - _cacheTime < 60000) {
    return _cachedModels;
  }
  
  const res = await fetch("https://api.example.com/models");
  _cachedModels = await res.json();
  _cacheTime = Date.now();
  return _cachedModels;
}
