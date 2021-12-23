import Wall from '@/components/canvas/Wall'
import dynamic from 'next/dynamic'
import { useState } from 'react'

const BubbleDispenser = dynamic(
  () => import('@/components/canvas/BubbleDispenser'),
  {
    ssr: false,
  }
)

interface Config {
  numberOfBubbles: number
  velocityFactor: number
}

interface ConfigControler extends Config {
  setNumberOfBubbles: (amount: number) => void
  setVelocityFactor: (factor: number) => void
}
// dom components goes here
const DOM: React.FC<ConfigControler> = ({
  numberOfBubbles,
  velocityFactor,
  setNumberOfBubbles,
  setVelocityFactor,
}) => {
  return (
    <div>
      <input
        type='number'
        value={numberOfBubbles}
        min='0'
        onChange={({ target: { value } }) => setNumberOfBubbles(Number(value))}
      />
      <input
        type='number'
        value={velocityFactor}
        min='0'
        onChange={({ target: { value } }) => setVelocityFactor(Number(value))}
      />
    </div>
  )
}
// canvas components goes here
const R3F: React.FC<Config> = ({ numberOfBubbles, velocityFactor }) => {
  return (
    <>
      <Wall />
      <BubbleDispenser
        numberOfBubbles={numberOfBubbles}
        velocityFactor={velocityFactor}
      />
      <textBufferGeometry />
    </>
  )
}

const Page = () => {
  const [numberOfBubbles, setNumberOfBubbles] = useState(2)
  const [velocityFactor, setVelocityFactor] = useState(1)
  return (
    <>
      <DOM
        numberOfBubbles={numberOfBubbles}
        velocityFactor={velocityFactor}
        setNumberOfBubbles={setNumberOfBubbles}
        setVelocityFactor={setVelocityFactor}
      />
      <R3F
        /* @ts-ignore */
        r3f
        numberOfBubbles={numberOfBubbles}
        velocityFactor={velocityFactor}
      />
    </>
  )
}

export default Page

export async function getStaticProps() {
  return {
    props: {
      title: 'Index',
    },
  }
}
