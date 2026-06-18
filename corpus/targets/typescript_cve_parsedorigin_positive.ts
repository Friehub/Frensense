  // ensure websocket requests from allowed origin

  if (rawOrigin && rawOrigin !== 'null') {
    const parsedOrigin = parseUrl(rawOrigin)

    if (parsedOrigin) {
      const originLowerCase = parsedOrigin.hostname.toLowerCase()

      if (!isCsrfOriginAllowed(originLowerCase, allowedOrigins)) {
        return warnOrBlockRequest(res, originLowerCase, mode)
      }
    }
  }

  return false