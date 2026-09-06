// [frensense]
// observation: A record is updated using only the record ID from user input, without validating that the authenticated user owns or is authorized to modify the record.
// impact: Attackers can modify records belonging to other users (Insecure Direct Object Reference).
// improvement: Include the authenticated user's ID in the update query criteria to ensure ownership.
// cwe: CWE-290
// cvss: 7.5
// owasp: A07:2021
// frensense-sink: update

import { type Request, type Response, type NextFunction } from 'express'

export function updateProductReviews() {
  return (req: Request, res: Response, next: NextFunction) => {
    const user = req.user; // Authenticated user
    // VULNERABILITY: Updates based only on req.body.id without checking user.id
    db.reviewsCollection.update(
      { _id: req.body.id },
      { $set: { message: req.body.message } }
    ).then((result: any) => {
      res.json(result)
    }).catch((err: Error) => {
      res.status(500).json(err)
    })
  }
}
