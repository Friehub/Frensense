      const cacheDir = join(/* turbopackIgnore: true */ this.cacheDir, cacheKey)
      const files = await promises.readdir(cacheDir)
      for (const file of files) {
        const [maxAgeSt, expireAtSt, etag, upstreamEtag, extension] =
          file.split('.', 5)
        const buffer = await promises.readFile(
          /* turbopackIgnore: true */ join(
            /* turbopackIgnore: true */ cacheDir,
            file
          )
        )
        const expireAt = Number(expireAtSt)
        const maxAge = Number(maxAgeSt)
        return {
          value: {
            kind: CachedRouteKind.IMAGE,
            etag,
            buffer,
            extension,
            upstreamEtag,
          },
          revalidateAfter:
            Math.max(maxAge, this.nextConfig.images.minimumCacheTTL) * 1000 +
            Date.now(),
          cacheControl: { revalidate: maxAge, expire: undefined },
          isStale: now > expireAt,
        }