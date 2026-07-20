// [frensense]
// observation: Headless UI Listbox options rendered via dangerouslySetInnerHTML with user-controlled data, enabling XSS.
// impact: Attacker-controlled content rendered as innerHTML executes arbitrary JavaScript in the application context.
// improvement: Render Listbox options using React children (text nodes) instead of dangerouslySetInnerHTML, or sanitize with DOMPurify.

import { Listbox } from "@headlessui/react";

interface Option {
  id: string;
  label: string;
}

interface ListboxProps {
  options: Option[];
  userHtml: string;
}

export function UnsafeListbox({ options, userHtml }: ListboxProps) {
  return (
    <Listbox>
      <Listbox.Button>Select</Listbox.Button>
      <Listbox.Options>
        {options.map((opt) => (
          <Listbox.Option key={opt.id} value={opt.id}>
            <span dangerouslySetInnerHTML={{ __html: userHtml }} />
          </Listbox.Option>
        ))}
      </Listbox.Options>
    </Listbox>
  );
}
