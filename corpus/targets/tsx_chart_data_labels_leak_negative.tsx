// SAFE: Masks PII fields before rendering chart labels, only showing anonymized identifiers
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

function maskEmail(email: string): string {
  const [name, domain] = email.split("@");
  return `${name[0]}***@${domain}`;
}

function maskSsn(ssn: string): string {
  return `***-**-${ssn.slice(-4)}`;
}

export function UserDataChart({ users }: ChartProps) {
  const data = users.map((u) => ({
    name: `User ${u.id}`,
    value: u.value,
  }));
  return (
    <PieChart width={400} height={400}>
      <Pie data={data} dataKey="value" nameKey="name" label />
    </PieChart>
  );
}
