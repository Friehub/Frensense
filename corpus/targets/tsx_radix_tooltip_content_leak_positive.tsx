// [frensense]
// observation: A Radix UI Tooltip displays sensitive server-side data (e.g., full SSN, internal API keys, database IDs) on hover, exposing it to any user who hovers over the trigger element.
// impact: Sensitive information is leaked via the tooltip on hover. Shoulder surfing, screen recording, or accidental hovers expose PII, internal identifiers, or secrets to unauthorized users or observers.
// improvement: Never place sensitive data in tooltip content. Mask or truncate the data, or use tooltips only for non-sensitive metadata with a separate "show details" action for sensitive information.

import * as Tooltip from '@radix-ui/react-tooltip';

interface UserRowProps {
  name: string;
  email: string;
  ssn: string;
  internalNotes: string;
}

export function UserRow({ name, email, ssn, internalNotes }: UserRowProps) {
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
                  <p>SSN: {ssn}</p>
                  <p>Notes: {internalNotes}</p>
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
