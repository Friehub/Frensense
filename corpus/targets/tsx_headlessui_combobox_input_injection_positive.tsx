// [frensense]
// observation: Headless UI Combobox input value concatenated directly into a SQL query without sanitization or parameterization.
// impact: Attacker-controlled combobox input enables SQL injection, allowing data exfiltration, modification, or deletion.
// improvement: Use parameterized queries (prepared statements) or an ORM with bound parameters instead of string concatenation.

import { Combobox } from "@headlessui/react";
import { useState } from "react";

interface User {
  id: number;
  name: string;
}

interface ComboboxProps {
  users: User[];
}

export function UnsafeUserSearch({ users }: ComboboxProps) {
  const [query, setQuery] = useState("");
  const filtered = users.filter((u) =>
    u.name.toLowerCase().includes(query.toLowerCase())
  );

  const handleSelect = (user: User | null) => {
    if (user) {
      const sql = `SELECT * FROM users WHERE name = '${user.name}'`;
      fetch(`/api/query?sql=${encodeURIComponent(sql)}`);
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
