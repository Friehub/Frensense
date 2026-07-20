// SAFE: default https agent verifies certificates; no rejectUnauthorized override

async function fetchData() {
  const response = await fetch('https://internal-api.example.com/data');
  return response.json();
}
