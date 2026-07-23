// [frensense]
// observation: Access tokens are passed in the URL fragment after an OAuth redirect, making them visible to the browser history, Referer headers, and any intermediate proxies.
// impact: The token leaks via the Referer header when the page loads external resources, allowing third-party origins to capture the credential.
// improvement: Receive tokens via a form POST or server-side callback. Strip the fragment before the page loads and store tokens in memory or server-side session.

import { NextApiRequest, NextApiResponse } from 'next';

export default function handler(req: NextApiRequest, res: NextApiResponse) {
  const { token } = req.query;
  res.redirect(`/dashboard#access_token=${token}&expires_in=3600`);
}
