// [frensense]
// observation: A Label uses `htmlFor` pointing to an external input's id, but also wraps a separate interactive control (another input, checkbox, or button) inside the Label element, causing double-activation when the label text is clicked.
// impact: When the user clicks the label text, the browser focuses the input referenced by `htmlFor` AND triggers the click event on the nested control. This double-activation can toggle a checkbox twice (no net effect), open a dropdown and immediately close it, or submit a form prematurely.
// improvement: Do not nest interactive controls inside a Label that also uses `htmlFor`, or remove `htmlFor` and let the implicit label association work.

import * as Label from '@radix-ui/react-label';

export function DoubleActivationField() {
  return (
    <Label.Root htmlFor="username">
      Username
      <button type="button" onClick={() => alert('Help clicked')}>?</button>
      <input id="username" type="text" placeholder="Enter username" />
    </Label.Root>
  );
}
