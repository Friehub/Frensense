// [frensense]
// observation: The push event reads `event.data.text()` and passes it directly into `self.registration.showNotification()` without any sanitization or validation. An attacker who controls the push service or intercepts the push message can inject arbitrary content into the notification.
// impact: An attacker can inject malicious content (XSS via notification body/title) or spoof the notification to appear as a legitimate system prompt, tricking the user into revealing credentials or clicking malicious links.
// improvement: Sanitize all text from push event data before displaying it in notifications. Use template-based notifications with only non-sensitive parameters.

self.addEventListener('push', (event: PushEvent) => {
  const payload = event.data?.text() ?? 'Default message';

  event.waitUntil(
    self.registration.showNotification('New Update', {
      body: payload,
      icon: '/icon.png',
    })
  );
});
