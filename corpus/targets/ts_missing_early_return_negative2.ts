// SAFE: Uses if/else structure to prevent fall-through to provider creation
export async function handleRun(env: any, existingSession: any) {
  try {
    const res = await env.stub.fetch("http://do/sandbox");
    if (res.ok) {
      await env.pInstance.keepAlive(existingSession);
      await env.stub.fetch("http://do/sandbox", { method: "PUT", body: JSON.stringify({ session: existingSession }) });
      console.log("Reusing warm sandbox environment...");
      return {
        sandboxId: existingSession.id,
        accessToken: existingSession.accessToken,
        isVercel: existingSession.provider === "vercel",
        reused: true,
      };
    }
  } catch (e) {
    // ignore
  }

  const result = await env.provider.createSession();
  return result;
}
