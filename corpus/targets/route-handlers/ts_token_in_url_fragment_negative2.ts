// SAFE: Fragment is stripped server-side via a POST-redirect pattern (form auto-submit)
import { NextApiRequest, NextApiResponse } from 'next';

export default function handler(req: NextApiRequest, res: NextApiResponse) {
  const { token } = req.query;
  res.status(200).send(`
    <html><body>
    <form id="f" action="/callback" method="POST">
      <input name="token" value="${token}" type="hidden"/>
    </form>
    <script>document.getElementById('f').submit()</script>
    </body></html>
  `);
}
