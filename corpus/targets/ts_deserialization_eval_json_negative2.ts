// SAFE alternative: JSON.parse with reviver for validation
function parseJsonData(data: string): any {
  return JSON.parse(data, (key, value) => {
    if (typeof value === 'string' && value.length > 10000) {
      throw new Error('String value too large');
    }
    return value;
  });
}
