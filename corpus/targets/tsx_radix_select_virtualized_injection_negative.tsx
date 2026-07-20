// SAFE: Labels are rendered as text content via curly braces, preventing any HTML injection

import * as Select from '@radix-ui/react-select';
import { CheckIcon, ChevronDownIcon } from '@radix-ui/react-icons';

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
                <Select.ItemText>{opt.label}</Select.ItemText>
                <Select.ItemIndicator><CheckIcon /></Select.ItemIndicator>
              </Select.Item>
            ))}
          </Select.Viewport>
        </Select.Content>
      </Select.Portal>
    </Select.Root>
  );
}
