import { useFrame } from '@react-three/fiber'
import { useRef, useState } from 'react'

const Bubble = ({ position, initialUpwards }) => {
  const [upwards, setUpwards] = useState(initialUpwards)
  useFrame((state, delta) => {
    if (mesh.current) {
      if (mesh.current.position.y > 4 && upwards) {
        setUpwards(false)
      }

      if (mesh.current.position.y < -4 && !upwards) {
        setUpwards(true)
      }

      if (upwards) {
        mesh.current.position.y += 0.05
      } else {
        mesh.current.position.y = mesh.current.position.y - 0.05
      }
    }
  })
  // This reference will give us direct access to the THREE.Mesh object
  const mesh = useRef(null)
  return (
    <>
      <mesh ref={mesh} position={position} scale={0.5}>
        <sphereBufferGeometry args={[1, 16, 16]} />
        <meshPhysicalMaterial color='#0af' />
      </mesh>
    </>
  )
}
export default Bubble
