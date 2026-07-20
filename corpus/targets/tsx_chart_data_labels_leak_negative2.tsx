// SAFE: Uses aggregated data instead of individual user records, preventing PII exposure
import { BarChart, Bar, XAxis, YAxis, Tooltip } from "recharts";

interface AggregateData {
  range: string;
  count: number;
}

export function AggregateChart() {
  const data: AggregateData[] = [
    { range: "0-100", count: 45 },
    { range: "101-200", count: 32 },
    { range: "201-300", count: 18 },
  ];
  return (
    <BarChart width={400} height={300} data={data}>
      <XAxis dataKey="range" />
      <YAxis />
      <Tooltip />
      <Bar dataKey="count" fill="#8884d8" />
    </BarChart>
  );
}
