    const urlParts = (req.url || '').split('?')
    const urlNoQuery = urlParts[0]

    if (urlNoQuery?.match(/(\\|\/\/)/)) {
      const cleanUrl = normalizeRepeatedSlashes(req.url!)
      res.setHeader('Location', cleanUrl)
      res.setHeader('Refresh', `0;url=${cleanUrl}`)
      res.statusCode = 308
      res.end(cleanUrl)
      return
    }
    setLazyProp({ req: req as any }, 'cookies', getCookieParser(req.headers))
    // Parse url if parsedUrl not provided
    if (!parsedUrl || typeof parsedUrl !== 'object') {
      parsedUrl = parseUrl(req.url!, true)
    }
    // Parse the querystring ourselves if the user doesn't handle querystring parsing
    if (typeof parsedUrl.query === 'string') {
      parsedUrl.query = parseQs(parsedUrl.query)
    }
    // When there are hostname and port we build an absolute URL
    const initUrl =
      this.hostname && this.port
        ? `http://${this.hostname}:${this.port}${req.url}`
        : req.url
    addRequestMeta(req, '__NEXT_INIT_URL', initUrl)
    addRequestMeta(req, '__NEXT_INIT_QUERY', { ...parsedUrl.query })
    const url = parseNextUrl({
      headers: req.headers,
      nextConfig: this.nextConfig,
      url: req.url?.replace(/^\/+/, '/'),
    })
    if (url.basePath) {
      req.url = replaceBasePath(req.url!, this.nextConfig.basePath)
      addRequestMeta(req, '_nextHadBasePath', true)
    }
    if (
      this.minimalMode &&
      req.headers['x-matched-path'] &&
      typeof req.headers['x-matched-path'] === 'string'
    ) {
      const reqUrlIsDataUrl = req.url?.includes('/_next/data')
      const matchedPathIsDataUrl =
        req.headers['x-matched-path']?.includes('/_next/data')
      const isDataUrl = reqUrlIsDataUrl || matchedPathIsDataUrl

      let parsedPath = parseUrl(
        isDataUrl ? req.url! : (req.headers['x-matched-path'] as string),
        true
      )
      let matchedPathname = parsedPath.pathname!
      let matchedPathnameNoExt = isDataUrl
        ? matchedPathname.replace(/\.json$/, '')
        : matchedPathname
      if (this.nextConfig.i18n) {
        const localePathResult = normalizeLocalePath(
          matchedPathname || '/',
          this.nextConfig.i18n.locales
        )
        if (localePathResult.detectedLocale) {
          parsedUrl.query.__nextLocale = localePathResult.detectedLocale
      }

      if (isDataUrl) {
        matchedPathname = denormalizePagePath(matchedPathname)
        matchedPathnameNoExt = denormalizePagePath(matchedPathnameNoExt)
      }
      const pageIsDynamic = isDynamicRoute(matchedPathnameNoExt)
      const combinedRewrites: Rewrite[] = []
      combinedRewrites.push(...this.customRoutes.rewrites.beforeFiles)
      combinedRewrites.push(...this.customRoutes.rewrites.afterFiles)
      combinedRewrites.push(...this.customRoutes.rewrites.fallback)
      const utils = getUtils({
        pageIsDynamic,
        page: matchedPathnameNoExt,
        i18n: this.nextConfig.i18n,
        basePath: this.nextConfig.basePath,
        rewrites: combinedRewrites,
      })
      try {
        // ensure parsedUrl.pathname includes URL before processing
        // rewrites or they won't match correctly
        if (this.nextConfig.i18n && !url.locale?.path.detectedLocale) {
          parsedUrl.pathname = `/${url.locale?.locale}${parsedUrl.pathname}`
        }
        utils.handleRewrites(req, parsedUrl)
        // interpolate dynamic params and normalize URL if needed
        if (pageIsDynamic) {
          let params: ParsedUrlQuery | false = {}
          Object.assign(parsedUrl.query, parsedPath.query)
          const paramsResult = utils.normalizeDynamicRouteParams(
            parsedUrl.query
          )
          if (paramsResult.hasValidParams) {
            params = paramsResult.params
          } else if (req.headers['x-now-route-matches']) {
            const opts: Record<string, string> = {}
            params = utils.getParamsFromRouteMatches(
              req,
              opts,
              parsedUrl.query.__nextLocale || ''
            if (opts.locale) {
              parsedUrl.query.__nextLocale = opts.locale
          } else {
            params = utils.dynamicRouteMatcher!(matchedPathnameNoExt)
          }
          if (params) {
            params = utils.normalizeDynamicRouteParams(params).params
            matchedPathname = utils.interpolateDynamicPath(
              matchedPathname,
              params
            )
            req.url = utils.interpolateDynamicPath(req.url!, params)

          if (reqUrlIsDataUrl && matchedPathIsDataUrl) {
            req.url = formatUrl({
              ...parsedPath,
              pathname: matchedPathname,
            })

          Object.assign(parsedUrl.query, params)
          utils.normalizeVercelUrl(req, true)
        }
      } catch (err) {
        if (err instanceof DecodeError) {
          res.statusCode = 400
          return this.renderError(null, req, res, '/_error', {})
        throw err
      }
      parsedUrl.pathname = `${this.nextConfig.basePath || ''}${
        matchedPathname === '/' && this.nextConfig.basePath
          ? ''
          : matchedPathname
      }`
      url.pathname = parsedUrl.pathname
    }
    addRequestMeta(req, '__nextHadTrailingSlash', url.locale?.trailingSlash)
    if (url.locale?.domain) {
      addRequestMeta(req, '__nextIsLocaleDomain', true)
    }
    if (url.locale?.path.detectedLocale) {
      req.url = formatUrl(url)
      addRequestMeta(req, '__nextStrippedLocale', true)
      if (url.pathname === '/api' || url.pathname.startsWith('/api/')) {
        return this.render404(req, res, parsedUrl)
    }
    if (!this.minimalMode || !parsedUrl.query.__nextLocale) {
      if (url?.locale?.locale) {
        parsedUrl.query.__nextLocale = url.locale.locale
    }
    if (url?.locale?.defaultLocale) {
      parsedUrl.query.__nextDefaultLocale = url.locale.defaultLocale
    }
    if (url.locale?.redirect) {
      res.setHeader('Location', url.locale.redirect)
      res.statusCode = TEMPORARY_REDIRECT_STATUS
      res.end()
      return
    }
    res.statusCode = 200
    try {
    } catch (err) {