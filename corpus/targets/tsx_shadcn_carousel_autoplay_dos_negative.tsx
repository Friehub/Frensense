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

const MIN_INTERVAL = 2000

export default function AutoplayCarousel() {
  const [interval, setInterval] = useState(3000)

  function handleIntervalChange(e: React.ChangeEvent<HTMLInputElement>) {
    const value = Number(e.target.value)
    // SAFE: enforce minimum interval to prevent CPU exhaustion
    setInterval(Math.max(MIN_INTERVAL, value))
  }

  return (
    <div>
      <input
        type="number"
        value={interval}
        onChange={handleIntervalChange}
        placeholder="Autoplay interval (ms)"
      />
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
