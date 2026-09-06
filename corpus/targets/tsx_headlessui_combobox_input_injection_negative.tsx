// SAFE: Uses parameterized query API instead of string concatenation for SQL
import { Combobox } from "@headlessui/react";
import { useState } from "react";

interface User {
  id: number;
  name: string;
}

interface ComboboxProps {
  users: User[];
}

export function SafeUserSearch({ users }: ComboboxProps) {
  const [query, setQuery] = useState("");
  const filtered = users.filter((u) =>
    u.name.toLowerCase().includes(query.toLowerCase())
  );

  const handleSelect = (user: User | null) => {
    if (user) {
      fetch("/api/users/search", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ name: user.name }),
      });
    }
  };

  return (
    <Combobox onChange={handleSelect}>
      <Combobox.Input onChange={(e) => setQuery(e.target.value)} />
      <Combobox.Options>
        {filtered.map((user) => (
          <Combobox.Option key={user.id} value={user}>
            {user.name}
          </Combobox.Option>
        ))}
      </Combobox.Options>
    </Combobox>
  );
}
