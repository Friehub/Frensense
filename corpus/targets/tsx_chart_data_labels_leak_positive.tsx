// [frensense]
// observation: Chart data labels in Recharts render full document object including PII fields (email, ssn) directly in DOM.
// impact: Sensitive user data is exposed in the DOM as chart labels, accessible to any script or browser extension inspecting the page.
// improvement: Truncate or mask sensitive fields before rendering, or use aggregate summaries instead of raw data.
// cwe: CWE-200
// cvss: 5.3
// owasp: 
// severity: Medium

import { PieChart, Pie, Cell, Tooltip } from "recharts";

interface UserData {
  id: number;
  email: string;
  ssn: string;
  value: number;
}

interface ChartProps {
  users: UserData[];
}

export function UserDataChart({ users }: ChartProps) {
  const data = users.map((u) => ({
    name: `${u.email} - ${u.ssn}`,
    value: u.value,
  }));
  return (
    <PieChart width={400} height={400}>
      <Pie data={data} dataKey="value" nameKey="name" label />
    </PieChart>
  );
}
