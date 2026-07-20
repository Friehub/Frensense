// SAFE: Uses crypto.randomUUID to generate secure tokens
function generateResetToken() {
  return crypto.randomUUID().replace(/-/g, "");
}
