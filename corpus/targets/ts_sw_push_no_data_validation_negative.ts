// SAFE: Sanitize push data by stripping HTML/malicious content and validating structure.

function sanitizeText(input: string): string {
  return input.replaceAll(/<[^>]*>/g, '').slice(0, 200);
}

function parsePayload(data: string): { title: string; body: string } | null {
  try {
    const parsed = JSON.parse(data);
    if (typeof parsed.title === 'string' && typeof parsed.body === 'string') {
      return { title: sanitizeText(parsed.title), body: sanitizeText(parsed.body) };
    }
    return null;
  } catch {
    return null;
  }
}

self.addEventListener('push', (event: PushEvent) => {
  const raw = event.data?.text() ?? '';
  const payload = parsePayload(raw);

  event.waitUntil(
    self.registration.showNotification(payload?.title ?? 'Default', {
      body: payload?.body ?? '',
      icon: '/icon.png',
    })
  );
});
