import { useThree } from '@react-three/fiber'
import { useRef } from 'react'

const Wall = () => {
  const mesh = useRef(null)
  return (
    <>
      <mesh ref={mesh} position={[0, 0, -0.5]}>
        <boxBufferGeometry args={[256, 256, 0]} />
        <meshPhongMaterial color='#222' />
      </mesh>
    </>
  )
}
export default Wall
