async function proxy(url: string) {
    const response = await fetch(url);
    return await response.text();
}
