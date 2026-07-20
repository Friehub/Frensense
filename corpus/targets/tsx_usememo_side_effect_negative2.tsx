// SAFE: memoized callback for side effects separated from the computation

import { useCallback, useEffect, useMemo, useState } from 'react';

export function AnalyticsDashboard() {
  const [events, setEvents] = useState<Array<{ page: string }>>([]);
  const [filter, setFilter] = useState('home');

  const onFilterChange = useCallback((newFilter: string) => {
    console.log('Filter changed:', newFilter);
    localStorage.setItem('lastFilter', newFilter);
    setFilter(newFilter);
  }, []);

  const filteredEvents = useMemo(() => {
    return events.filter(e => e.page === filter);
  }, [events, filter]);

  return <div>{filteredEvents.length} events</div>;
}
