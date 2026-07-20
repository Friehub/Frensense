// SAFE: Label does not use htmlFor — implicit association via wrapping works without conflicting with the nested help button

import * as Label from '@radix-ui/react-label';

export function DoubleActivationField() {
  return (
    <Label.Root>
      Username
      <button type="button" onClick={() => alert('Help clicked')} style={{ marginLeft: 4 }}>?</button>
      <input type="text" placeholder="Enter username" />
    </Label.Root>
  );
}
