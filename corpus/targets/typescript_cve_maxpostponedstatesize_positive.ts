            // Get the configured max postponed state size.
            const maxPostponedStateSize =
              this.nextConfig.experimental.maxPostponedStateSize ??
              DEFAULT_MAX_POSTPONED_STATE_SIZE
            const maxPostponedStateSizeBytes = parseMaxPostponedStateSize(
              this.nextConfig.experimental.maxPostponedStateSize
            )
            if (maxPostponedStateSizeBytes === undefined) {
              throw new Error(
                'maxPostponedStateSize must be a valid number (bytes) or filesize format string (e.g., "5mb")'
            }
            const body: Array<Buffer> = []
            let size = 0
            for await (const chunk of req.body) {
              size += Buffer.byteLength(chunk)
              if (size > maxPostponedStateSizeBytes) {
                res.statusCode = 413
                const errorMessage =
                  `Postponed state exceeded ${maxPostponedStateSize} limit. ` +
                  `To configure the limit, see: https://nextjs.org/docs/app/api-reference/config/next-config-js/max-postponed-state-size`
                res.body(errorMessage).send()
                return
              }
              body.push(chunk)
            const postponed = Buffer.concat(body).toString('utf8')