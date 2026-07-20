// [frensense]
// observation: shadcn carousel autoplay interval is set from user-controlled parameters without validation, allowing extremely fast intervals that cause CPU exhaustion
// impact: denial-of-service (CPU exhaustion) — attacker sets interval to 0ms or 1ms, causing rapid re-renders that freeze the browser tab
// improvement: enforce a minimum interval value (e.g., 2000ms) for autoplay

'use client'

import { useState } from 'react'
import {
  Carousel,
  CarouselContent,
  CarouselItem,
} from '@/components/ui/carousel'

const VALID_INTERVALS = [2000, 3000, 5000, 10000]

export default function AutoplayCarousel() {
  const [interval, setInterval] = useState(3000)

  function handleIntervalChange(e: React.ChangeEvent<HTMLSelectElement>) {
    // SAFE: only allow predefined interval values, no arbitrary user input
    setInterval(Number(e.target.value))
  }

  return (
    <div>
      <select value={interval} onChange={handleIntervalChange}>
        {VALID_INTERVALS.map((ms) => (
          <option key={ms} value={ms}>{ms}ms</option>
        ))}
      </select>
      <Carousel opts={{ loop: true, align: 'start' }}>
        <CarouselContent autoplayInterval={interval}>
          <CarouselItem>Slide 1</CarouselItem>
          <CarouselItem>Slide 2</CarouselItem>
          <CarouselItem>Slide 3</CarouselItem>
        </CarouselContent>
      </Carousel>
    </div>
  )
}
