// SAFE: Z-index values are drawn from a predefined set of constants. User input cannot influence z-index stacking.

const Z_LAYERS = {
  dropdown: 50,
  modal: 100,
  toast: 150,
  overlay: 200,
} as const;

type ZLayer = keyof typeof Z_LAYERS;

export function OverlayAd({ layer = 'overlay' }: { layer?: ZLayer }) {
  const zIndex = Z_LAYERS[layer] ?? Z_LAYERS.overlay;

  return (
    <div
      className="fixed inset-0 flex items-center justify-center bg-white/90"
      style={{ zIndex }}
    >
      <div className="p-6 bg-white rounded-lg shadow-2xl">
        <h2>Welcome!</h2>
        <button className="bg-blue-500 text-white px-6 py-2 rounded">
          Continue
        </button>
      </div>
    </div>
  );
}
