  const allowedOrigins = [
    'localhost',
    ...(config.experimental.allowedDevOrigins || []),
  ]