// SAFE: Uses a fixed number of individual job adds instead of repeatable jobs

import Queue from 'bull';

const emailQueue = new Queue('email', 'redis://localhost:6379');

async function scheduleBatchNewsletters() {
  const dates = generateWeeklyDates(52);
  for (const date of dates) {
    await emailQueue.add(
      { type: 'weekly_newsletter', scheduledFor: date.toISOString() },
      { delay: date.getTime() - Date.now() }
    );
  }
}
