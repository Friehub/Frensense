// SAFE: Sanitizes user HTML with DOMPurify before passing to dangerouslySetInnerHTML
import { Listbox } from "@headlessui/react";
import DOMPurify from "dompurify";

interface Option {
  id: string;
  label: string;
}

interface ListboxProps {
  options: Option[];
  userHtml: string;
}

export function SafeListbox({ options, userHtml }: ListboxProps) {
  const sanitizedHtml = DOMPurify.sanitize(userHtml);
  return (
    <Listbox>
      <Listbox.Button>Select</Listbox.Button>
      <Listbox.Options>
        {options.map((opt) => (
          <Listbox.Option key={opt.id} value={opt.id}>
            <span dangerouslySetInnerHTML={{ __html: sanitizedHtml }} />
          </Listbox.Option>
        ))}
      </Listbox.Options>
    </Listbox>
  );
}
