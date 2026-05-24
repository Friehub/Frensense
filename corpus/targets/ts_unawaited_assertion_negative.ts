it('should pass if awaited', async () => {
    await expect(Promise.resolve(1)).resolves.toBe(1);
});
