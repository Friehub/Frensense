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

export default function AutoplayCarousel() {
  const [interval, setInterval] = useState(3000)

  return (
    <div>
      <input
        type="number"
        value={interval}
        onChange={(e) => setInterval(Number(e.target.value))}
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
