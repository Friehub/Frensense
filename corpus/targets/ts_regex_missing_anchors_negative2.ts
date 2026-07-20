// SAFE alternative: use length check + anchored regex
function isValidZipCode(zip: string): boolean {
  return zip.length === 5 && /^\d{5}$/.test(zip);
}

function isValidUsername(username: string): boolean {
  return username.length >= 3 && username.length <= 16 && /^[a-z0-9_]+$/.test(username);
}
