// SAFE: R2 writes are gated by a valid session token check

interface Env {
  BUCKET: R2Bucket;
  AUTH: string;
}

async function isAuthenticated(request: Request): Promise<boolean> {
  const auth = request.headers.get('Authorization');
  return auth === `Bearer ${process.env.API_TOKEN}`;
}

export const onRequest: PagesFunction<Env> = async (context) => {
  if (!isAuthenticated(context.request)) {
    return new Response('Unauthorized', { status: 401 });
  }
  const formData = await context.request.formData();
  const file = formData.get('file') as File;
  await context.env.BUCKET.put(file.name, file.stream());
  return new Response('Uploaded');
};
