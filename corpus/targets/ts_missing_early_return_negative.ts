export async function handleRun(env: any, existingSession: any) {
  try {
    const res = await env.stub.fetch("http://do/sandbox");
    if (res.ok) {
      await env.pInstance.keepAlive(existingSession);
      await env.stub.fetch("http://do/sandbox", { method: "PUT", body: JSON.stringify({ session: existingSession }) });
      console.log("Reusing warm sandbox environment...");
      // SAFE: Returns early
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

  const tryProvider = async () => {
    // always creates new sandbox
    return await env.provider.createSession();
  };

  const result = await tryProvider();
  return result;
}
