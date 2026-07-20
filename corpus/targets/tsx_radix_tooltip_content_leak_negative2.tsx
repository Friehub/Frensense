// SAFE: Sensitive data is only shown via a click-to-reveal pattern, not in the hover tooltip

import * as Tooltip from '@radix-ui/react-tooltip';
import { useState } from 'react';

interface UserRowProps {
  name: string;
  email: string;
  ssn: string;
}

export function UserRow({ name, email, ssn }: UserRowProps) {
  const [showSSN, setShowSSN] = useState(false);
  const maskedSSN = `***-**-${ssn.slice(-4)}`;

  return (
    <tr>
      <td>
        <Tooltip.Provider>
          <Tooltip.Root>
            <Tooltip.Trigger asChild>
              <span style={{ cursor: 'pointer' }}>{name}</span>
            </Tooltip.Trigger>
            <Tooltip.Portal>
              <Tooltip.Content side="top" sideOffset={4}>
                <div>
                  <p>Email: {email}</p>
                  <p>Role: User</p>
                </div>
                <Tooltip.Arrow />
              </Tooltip.Content>
            </Tooltip.Portal>
          </Tooltip.Root>
        </Tooltip.Provider>
      </td>
      <td>{email}</td>
      <td>
        <button onClick={() => setShowSSN(!showSSN)}>
          {showSSN ? ssn : maskedSSN}
        </button>
      </td>
    </tr>
  );
}
