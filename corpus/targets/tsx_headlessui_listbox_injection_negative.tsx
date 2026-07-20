// SAFE: Renders Listbox options using text content instead of dangerouslySetInnerHTML
import { Listbox } from "@headlessui/react";

interface Option {
  id: string;
  label: string;
}

interface ListboxProps {
  options: Option[];
}

export function SafeListbox({ options }: ListboxProps) {
  return (
    <Listbox>
      <Listbox.Button>Select</Listbox.Button>
      <Listbox.Options>
        {options.map((opt) => (
          <Listbox.Option key={opt.id} value={opt.id}>
            {opt.label}
          </Listbox.Option>
        ))}
      </Listbox.Options>
    </Listbox>
  );
}
