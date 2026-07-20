// SAFE: Stores user config in a JSON data attribute parsed by a trusted external script
interface UserWidgetProps {
  config: Record<string, unknown>;
}

export function UserWidget({ config }: UserWidgetProps) {
  return (
    <div
      class="widget-container"
      data-config={JSON.stringify(config)}
    >
      Widget content
    </div>
  );
}
