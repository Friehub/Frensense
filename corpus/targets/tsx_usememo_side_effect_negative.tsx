// SAFE: side effects are in useEffect; useMemo is purely computational

import { useEffect, useMemo, useState } from 'react';

export function AnalyticsDashboard() {
  const [events, setEvents] = useState<Array<{ page: string }>>([]);
  const [filter, setFilter] = useState('home');

  useEffect(() => {
    console.log('Filter changed:', filter);
    localStorage.setItem('lastFilter', filter);
  }, [filter]);

  const filteredEvents = useMemo(() => {
    return events.filter(e => e.page === filter);
  }, [events, filter]);

  return <div>{filteredEvents.length} events</div>;
}
