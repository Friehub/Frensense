// [frensense]
// observation: A Cloudflare Workers handler writes to R2 without verifying the caller's identity or permissions.
// impact: Unauthenticated users can upload arbitrary objects to R2 buckets, filling storage or overwriting existing data.
// improvement: Add authentication and authorization checks before every R2 put() or delete() operation.

interface Env {
  BUCKET: R2Bucket;
}

export const onRequest: PagesFunction<Env> = async (context) => {
  const formData = await context.request.formData();
  const file = formData.get('file') as File;
  await context.env.BUCKET.put(file.name, file.stream());
  return new Response('Uploaded');
};
