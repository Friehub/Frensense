// SAFE: Tooltip content masks sensitive data and only shows non-sensitive metadata

import * as Tooltip from '@radix-ui/react-tooltip';

interface UserRowProps {
  name: string;
  email: string;
  ssn: string;
}

export function UserRow({ name, email, ssn }: UserRowProps) {
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
                  <p>SSN: {maskedSSN}</p>
                  <p style={{ fontSize: '0.75rem', color: '#888' }}>Click to view full details</p>
                </div>
                <Tooltip.Arrow />
              </Tooltip.Content>
            </Tooltip.Portal>
          </Tooltip.Root>
        </Tooltip.Provider>
      </td>
      <td>{email}</td>
    </tr>
  );
}
