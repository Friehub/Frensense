// [frensense]
// observation: A useMemo callback performs side effects such as API calls, logging, or DOM mutations instead of pure computation.
// impact: Side effects in useMemo run during rendering, breaking React's purity contract. They may execute more or fewer times than expected, causing inconsistent behavior, performance issues, or subtle bugs when React discards the memoized value.
// improvement: Move side effects into a useEffect hook and use useMemo only for pure computations.

import { useMemo, useState } from 'react';

export function AnalyticsDashboard() {
  const [events, setEvents] = useState<Array<{ page: string }>>([]);
  const [filter, setFilter] = useState('home');

  const filteredEvents = useMemo(() => {
    console.log('Filtering events:', filter);
    localStorage.setItem('lastFilter', filter);
    return events.filter(e => e.page === filter);
  }, [events, filter]);

  return <div>{filteredEvents.length} events</div>;
}
