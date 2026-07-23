// SAFE: Uses async/await with try/catch instead of promise .catch()
async function fetchData() {
  try {
    const res = await fetch("/api/data");
    const data = await res.json();
    console.log(data);
  } catch (err) {
    console.error(err);
  }
}
