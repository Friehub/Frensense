      // Read the entire body, checking size as we go.
      const bodyChunks: Array<Buffer> = []
      let size = 0
      for await (const chunk of req) {
        const buffer = Buffer.isBuffer(chunk) ? chunk : Buffer.from(chunk)
        size += buffer.byteLength
        if (size > maxTotalBodySize) {
          res.statusCode = 413
          res.end(
            `Request body exceeded limit. ` +
              `To configure the body size limit for Server Actions, see: https://nextjs.org/docs/app/api-reference/next-config-js/serverActions#bodysizelimit`
          )
          ctx.waitUntil?.(Promise.resolve())
          return null
        }
        bodyChunks.push(buffer)
      const fullBody = Buffer.concat(bodyChunks)