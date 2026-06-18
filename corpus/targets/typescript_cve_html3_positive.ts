    const html3 = await renderViaHTTP(appPort, '/some-other-path', undefined, {
      headers: {
        'x-matched-path': '/dynamic/[slug]',
        'x-now-route-matches': '1=second&nxtPslug=second',
      },
    })