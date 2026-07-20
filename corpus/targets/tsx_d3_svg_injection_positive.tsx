// [frensense]
// observation: D3 SVG element attributes set from unsanitized user data, allowing XSS via onload or other event handler attributes.
// impact: Attacker injects SVG onload/onerror event handlers that execute arbitrary JavaScript in the application context.
// improvement: Sanitize SVG attributes and content using DOMPurify or use React's built-in JSX sanitization (avoid innerHTML/setAttribute with user data).

import * as d3 from "d3";
import { useEffect, useRef } from "react";

interface SvgProps {
  userLabel: string;
}

export function DynamicSvg({ userLabel }: SvgProps) {
  const svgRef = useRef<SVGSVGElement>(null);

  useEffect(() => {
    if (svgRef.current) {
      const svg = d3.select(svgRef.current);
      svg.append("text")
        .attr("x", 10)
        .attr("y", 20)
        .attr("onclick", userLabel)
        .text("Hello");
    }
  }, [userLabel]);

  return <svg ref={svgRef} width={200} height={100} />;
}
