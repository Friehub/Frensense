// SAFE: Repeatable job has an explicit limit to prevent infinite repetition

import Queue from 'bull';

const emailQueue = new Queue('email', 'redis://localhost:6379');

async function scheduleNewsletter() {
  await emailQueue.add(
    { type: 'weekly_newsletter' },
    {
      repeat: {
        cron: '0 8 * * 1',
        limit: 52,
      },
    }
  );
}
