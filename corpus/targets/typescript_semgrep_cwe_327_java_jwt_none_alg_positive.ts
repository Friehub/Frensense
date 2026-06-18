// Vulnerable: Detected use of the 'none' algorithm in a JWT token. The 'none' algorithm assumes the integrity of the token has already been verified. This would allow a malicious actor to forge a JWT token that will automatically be verified. Do not explicitly use the 'none' algorithm. Instead, use an algorithm such as 'HS256'.
// Pattern: {'pattern': '$JWT.sign(com.auth0.jwt.algorithms.Algorithm.none());\n'} | {'pattern': '$NONE = com.auth0.jwt.algorithms.Algorithm.none();\n...\n$JWT.sign($NONE);\n'} | {'pattern': 'class $CLASS {\n  ...\n  $TYPE $NONE = com.auth0.jwt.algorithms.Algorithm.none();\n  ...\n  $RETURNTYPE $FUNC (...) {\n    ...\n    $JWT.sign($NONE);\n    ...\n  }\n  ...\n}'}
function vulnerable() {
  // TODO: implement pattern match
}
