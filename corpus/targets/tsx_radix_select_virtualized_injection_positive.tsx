// [frensense]
// observation: A Select component renders items using user-provided data (e.g., from an API or URL params) without sanitizing the label text, allowing XSS via dangerouslySetInnerHTML or directly rendering HTML-like content.
// impact: An attacker who can control select item labels (via a compromised API, URL params, or stored data) can inject arbitrary HTML/JS into the page. When the select opens and renders the malicious item, the script executes in the application context, leading to session theft, data exfiltration, or full account takeover.
// improvement: Never use dangerouslySetInnerHTML with user data. Sanitize all user-controlled text with a library like DOMPurify, or render text only via React's text content (curly braces).

import * as Select from '@radix-ui/react-select';
import { CheckIcon, ChevronDownIcon } from '@radix-ui/react-icons';

interface VirtualizedOption {
  value: string;
  label: string;
}

export function UserProvidedSelect({ options }: { options: VirtualizedOption[] }) {
  return (
    <Select.Root>
      <Select.Trigger>
        <Select.Value placeholder="Pick an option" />
        <Select.Icon><ChevronDownIcon /></Select.Icon>
      </Select.Trigger>
      <Select.Portal>
        <Select.Content>
          <Select.Viewport>
            {options.map((opt) => (
              <Select.Item key={opt.value} value={opt.value}>
                <Select.ItemText>
                  <span dangerouslySetInnerHTML={{ __html: opt.label }} />
                </Select.ItemText>
                <Select.ItemIndicator><CheckIcon /></Select.ItemIndicator>
              </Select.Item>
            ))}
          </Select.Viewport>
        </Select.Content>
      </Select.Portal>
    </Select.Root>
  );
}
