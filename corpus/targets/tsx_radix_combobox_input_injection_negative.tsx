// SAFE: The user input is URL-encoded via encodeURIComponent before being interpolated into the API URL

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
    const encoded = encodeURIComponent(value);
    const results = await fetch(`/api/search?q=${encoded}`).then((r) => r.json());
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
