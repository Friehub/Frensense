async function proxy(url: string) {
    const parsed = new URL(url);
    if (parsed.hostname === "api.trusted.com") {
        const response = await fetch(url);
        return await response.text();
    }
    throw new Error("URL not allowed");
}
