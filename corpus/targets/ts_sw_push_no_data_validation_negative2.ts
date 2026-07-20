// SAFE: Use template-based notifications and only use push data as a non-displayed reference.

const NOTIFICATION_TEMPLATES = {
  newMessage: { title: 'New Message', body: 'You have a new message' },
  update: { title: 'Update Available', body: 'A new version is ready' },
} as const;

self.addEventListener('push', (event: PushEvent) => {
  const raw = event.data?.text() ?? '{}';
  let templateKey = 'update';
  try {
    const parsed = JSON.parse(raw);
    if (parsed.type && Object.hasOwn(NOTIFICATION_TEMPLATES, parsed.type)) {
      templateKey = parsed.type;
    }
  } catch {
    // invalid JSON, use default
  }

  const template = NOTIFICATION_TEMPLATES[templateKey as keyof typeof NOTIFICATION_TEMPLATES];
  event.waitUntil(
    self.registration.showNotification(template.title, {
      body: template.body,
      icon: '/icon.png',
    })
  );
});
