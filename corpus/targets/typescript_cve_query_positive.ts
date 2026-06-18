  it('should fail when url is recursive', async () => {
    const query = { url: `/_next/image?url=test.pngw=1&q=1`, w: ctx.w, q: 1 }
    const res = await fetchViaHTTP(ctx.appPort, '/_next/image', query, {})
    expect(res.status).toBe(400)
    expect(await res.text()).toBe(`"url" parameter cannot be recursive`)