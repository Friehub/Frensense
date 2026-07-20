// SAFE alternative: Promise.allSettled for concurrent processing
async function loadData(ids: number[]): Promise<any[]> {
  const promises = ids.map(id => fetch(`/api/item/${id}`).then(r => r.json()));
  const results = await Promise.allSettled(promises);
  return results.filter(r => r.status === 'fulfilled').map(r => (r as PromiseFulfilledResult<any>).value);
}
