  const originDomain =
    typeof originHeader === 'string' && originHeader !== 'null'
      ? new URL(originHeader).host