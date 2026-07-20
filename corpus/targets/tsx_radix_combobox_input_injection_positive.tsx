// [frensense]
// observation: A Combobox input value is concatenated directly into an API query string without sanitization or encoding, allowing an attacker to inject arbitrary parameters or SQL-like operators into the API call.
// impact: An attacker can type special characters (e.g., `&`, `=`, `'`) into the combobox to manipulate the API request, potentially performing SSRF, parameter pollution, or injection attacks against backend services. This can lead to unauthorized data access or account compromise.
// improvement: Always encode user input when interpolating into URLs, or use a library's built-in query parameter builder.

import * as React from 'react';
import { CheckIcon, ChevronDownIcon } from '@radix-ui/react-icons';

interface ComboboxOption {
  value: string;
  label: string;
}

export function SearchableCombobox({ onSearch }: { onSearch: (query: string) => Promise<ComboboxOption[]> }) {
  const [inputValue, setInputValue] = React.useState('');
  const [options, setOptions] = React.useState<ComboboxOption[]>([]);

  const handleSearch = async (value: string) => {
    setInputValue(value);
    const results = await fetch(`/api/search?q=${value}`).then((r) => r.json());
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
