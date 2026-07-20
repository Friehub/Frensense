// SAFE: Uses a hash chain to make audit logs tamper-evident (blockchain-inspired)
import { createClient } from 'redis';
import crypto from 'crypto';

const redis = createClient();

export async function appendAuditLog(entry: AuditEntry): Promise<void> {
  const lastHash = await redis.get('audit:lastHash') || '0'.repeat(64);
  const hash = crypto.createHash('sha256').update(lastHash + JSON.stringify(entry)).digest('hex');
  await redis.xAdd('audit:stream', '*', { hash, data: JSON.stringify(entry), prevHash: lastHash });
  await redis.set('audit:lastHash', hash);
}

export async function verifyAuditChain(): Promise<boolean> {
  const entries = await redis.xRange('audit:stream', '-', '+');
  let prevHash = '0'.repeat(64);
  for (const entry of entries) {
    const { hash, data, prevHash: claimedPrev } = entry.message;
    if (claimedPrev !== prevHash) return false;
    const computed = crypto.createHash('sha256').update(prevHash + data).digest('hex');
    if (hash !== computed) return false;
    prevHash = hash;
  }
  return true;
}

interface AuditEntry {
  userId: string;
  action: string;
  resource: string;
  timestamp: string;
}
