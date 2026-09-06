// [frensense]
// observation: The delete endpoint accepts a resource ID from the user and deletes it without verifying that the user owns that resource.
// impact: An authenticated user can delete any resource in the system by guessing or enumerating resource IDs, leading to mass data loss.
// improvement: Verify resource ownership before performing the delete operation.
// cwe: CWE-639
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: idor

export async function deleteDocument(req: Request, db: DB): Promise<Response> {
  const docId = req.params.id;
  await db.prepare('DELETE FROM documents WHERE id = ?').bind(docId).run();
  return new Response(JSON.stringify({ deleted: true }));
}

export async function deleteComment(req: Request, db: DB): Promise<Response> {
  const { commentId } = req.body;
  await db.prepare('DELETE FROM comments WHERE id = ?').bind(commentId).run();
  return new Response('Deleted');
}
