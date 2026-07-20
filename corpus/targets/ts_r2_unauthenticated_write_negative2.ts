// SAFE: File uploads are scoped to the authenticated user's directory prefix in R2

interface Env {
  BUCKET: R2Bucket;
}

function getUserId(request: Request): string | null {
  const token = request.headers.get('Authorization')?.replace('Bearer ', '');
  return token ? verifyToken(token) : null;
}

export const onRequest: PagesFunction<Env> = async (context) => {
  const userId = getUserId(context.request);
  if (!userId) return new Response('Unauthorized', { status: 401 });

  const formData = await context.request.formData();
  const file = formData.get('file') as File;
  await context.env.BUCKET.put(`uploads/${userId}/${file.name}`, file.stream());
  return new Response('Uploaded');
};
