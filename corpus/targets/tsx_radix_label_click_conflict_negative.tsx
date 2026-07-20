// SAFE: The label uses htmlFor for the input while the help button is placed outside the label to avoid double-activation

import * as Label from '@radix-ui/react-label';

export function DoubleActivationField() {
  return (
    <div>
      <Label.Root htmlFor="username">Username</Label.Root>
      <button type="button" onClick={() => alert('Help clicked')} aria-label="Help for username">?</button>
      <input id="username" type="text" placeholder="Enter username" />
    </div>
  );
}
