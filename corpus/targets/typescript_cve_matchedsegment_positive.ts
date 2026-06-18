    if (
      isRoutePPREnabled &&
      isPrefetchRSCRequest &&
      typeof segmentPrefetchHeader === 'string'
    ) {
      if (cacheEntry?.value?.kind === CachedRouteKind.APP_PAGE) {
        // This is a prefetch request for an individual segment's static data.
        // Unless the segment is fully dynamic, the data should have already been
        // loaded into the cache, when the page itself was generated. So we should
        // always either return the cache entry. If no cache entry is available,
        // it's a 404 — either the segment is fully dynamic, or an invalid segment
        // path was requested.
        if (cacheEntry.value.segmentData) {
          const matchedSegment = cacheEntry.value.segmentData.get(
            segmentPrefetchHeader
          )
          if (matchedSegment !== undefined) {
            return {
              type: 'rsc',
              body: RenderResult.fromStatic(matchedSegment),
              // TODO: Eventually this should use revalidate time of the
              // individual segment, not the whole page.
              revalidate: cacheEntry.revalidate,
            }
        // If the segment is not found, return a 404. Since this is an RSC
        // request, there's no reason to render a 404 page; just return an
        // empty response.
        res.statusCode = 404
        return {
          type: 'rsc',
          body: RenderResult.fromStatic(''),
          revalidate: cacheEntry.revalidate,
        }
      } else {
        // Segment prefetches should never reach the application layer. If
        // there's no cache entry for this page, it's a 404.
        res.statusCode = 404
        return {
          type: 'rsc',
          body: RenderResult.fromStatic(''),
        }