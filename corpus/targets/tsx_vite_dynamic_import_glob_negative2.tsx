// SAFE: Uses explicit static imports instead of glob, avoiding file structure exposure
import WidgetA from "../widgets/WidgetA";
import WidgetB from "../widgets/WidgetB";

const widgets = { WidgetA, WidgetB };

export function WidgetList() {
  return (
    <ul>
      {Object.keys(widgets).map((name) => (
        <li key={name}>{name}</li>
      ))}
    </ul>
  );
}
