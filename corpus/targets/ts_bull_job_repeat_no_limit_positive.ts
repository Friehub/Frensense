// [frensense]
// observation: "A repeatable job is added to the queue without a limit on the number of repetitions."
// impact: "The job repeats indefinitely, causing unbounded processing costs, queue build-up, and potential denial of service."
// improvement: "Specify a max limit on repeatable jobs, or use a finite number of repetitions."

import Queue from 'bull';

const emailQueue = new Queue('email', 'redis://localhost:6379');

async function scheduleNewsletter() {
  await emailQueue.add(
    { type: 'weekly_newsletter' },
    {
      repeat: {
        cron: '0 8 * * 1',
      },
    }
  );
}
