// SAFE: User-provided labels are sanitized through a function that escapes HTML entities before rendering

import * as Select from '@radix-ui/react-select';
import { CheckIcon, ChevronDownIcon } from '@radix-ui/react-icons';

function escapeHtml(text: string): string {
  return text.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');
}

interface SelectOption {
  value: string;
  label: string;
}

export function SafeSelect({ options }: { options: SelectOption[] }) {
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
                <Select.ItemText>{escapeHtml(opt.label)}</Select.ItemText>
                <Select.ItemIndicator><CheckIcon /></Select.ItemIndicator>
              </Select.Item>
            ))}
          </Select.Viewport>
        </Select.Content>
      </Select.Portal>
    </Select.Root>
  );
}
