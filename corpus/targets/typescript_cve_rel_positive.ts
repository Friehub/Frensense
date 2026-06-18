              const rel = getRelativeURL(value, initUrl)
              resHeaders['location'] = rel
              parsedUrl = url.parse(rel, true)
              return {
                parsedUrl,
                resHeaders,
                finished: true,
                statusCode: middlewareRes.status,