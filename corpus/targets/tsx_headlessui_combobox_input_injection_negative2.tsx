// SAFE: Uses a proper ORM-based API endpoint with parameterized queries internally
import { Combobox } from "@headlessui/react";
import { useState } from "react";

interface User {
  id: number;
  name: string;
}

interface ComboboxProps {
  onUserSelect: (user: User) => void;
}

export function SafeUserSearch({ onUserSelect }: ComboboxProps) {
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<User[]>([]);

  const handleQuery = async (value: string) => {
    setQuery(value);
    const res = await fetch(`/api/users?q=${encodeURIComponent(value)}`);
    const data = await res.json();
    setResults(data);
  };

  return (
    <Combobox onChange={onUserSelect}>
      <Combobox.Input onChange={(e) => handleQuery(e.target.value)} />
      <Combobox.Options>
        {results.map((user) => (
          <Combobox.Option key={user.id} value={user}>
            {user.name}
          </Combobox.Option>
        ))}
      </Combobox.Options>
    </Combobox>
  );
}
