import { useRef, useState } from 'react'
import Bubble from './Bubble'

const BubbleDispenser = ({ velocityFactor, numberOfBubbles }) => {
  return (
    <>
      <group>
        {Array.from(Array(numberOfBubbles).keys()).map((value) => (
          <Bubble
            key={value}
            position={[value - 7, 0, 0]}
            initialUpwards={true}
          />
        ))}
      </group>
    </>
  )
}
export default BubbleDispenser
