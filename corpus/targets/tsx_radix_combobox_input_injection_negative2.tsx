// SAFE: Uses URLSearchParams to properly build the query string, ensuring all user input is safely encoded

import * as React from 'react';

interface ComboboxOption {
  value: string;
  label: string;
}

export function SearchableCombobox({ onSearch }: { onSearch: (query: string) => Promise<ComboboxOption[]> }) {
  const [inputValue, setInputValue] = React.useState('');
  const [options, setOptions] = React.useState<ComboboxOption[]>([]);

  const handleSearch = async (value: string) => {
    setInputValue(value);
    const params = new URLSearchParams({ q: value });
    const results = await fetch(`/api/search?${params}`).then((r) => r.json());
    setOptions(results);
  };

  return (
    <div>
      <input
        role="combobox"
        aria-expanded
        value={inputValue}
        onChange={(e) => handleSearch(e.target.value)}
        placeholder="Search..."
      />
      <ul role="listbox">
        {options.map((opt) => (
          <li key={opt.value} role="option" aria-selected={false}>
            {opt.label}
          </li>
        ))}
      </ul>
    </div>
  );
}
