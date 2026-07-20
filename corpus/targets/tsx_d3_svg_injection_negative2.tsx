// SAFE: Sanitizes user label with DOMPurify before setting D3 text content
import * as d3 from "d3";
import { useEffect, useRef } from "react";
import DOMPurify from "dompurify";

interface SvgProps {
  userLabel: string;
}

export function DynamicSvg({ userLabel }: SvgProps) {
  const svgRef = useRef<SVGSVGElement>(null);

  useEffect(() => {
    if (svgRef.current) {
      const safeLabel = DOMPurify.sanitize(userLabel);
      const svg = d3.select(svgRef.current);
      svg.append("text")
        .attr("x", 10)
        .attr("y", 20)
        .text(safeLabel);
    }
  }, [userLabel]);

  return <svg ref={svgRef} width={200} height={100} />;
}
