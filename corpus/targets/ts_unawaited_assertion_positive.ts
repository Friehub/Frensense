it('should fail if unawaited', () => {
    expect(Promise.resolve(1)).resolves.toBe(1);
});
