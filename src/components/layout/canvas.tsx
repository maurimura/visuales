import { Canvas } from '@react-three/fiber'
import { OrbitControls, Preload } from '@react-three/drei'
import useStore from '@/helpers/store'
import { useEffect, useRef } from 'react'
import { KernelSize, Resizer } from 'postprocessing'
import { EffectComposer, Bloom } from '@react-three/postprocessing'

const LControl = () => {
  const dom = useStore((state) => state.dom)
  const control = useRef(null)

  useEffect(() => {
    if (control) {
      dom.current.style['touch-action'] = 'none'
    }
  }, [dom, control])
  // @ts-ignore
  return <OrbitControls ref={control} domElement={dom.current} />
}
const LCanvas = ({ children }) => {
  const dom = useStore((state) => state.dom)

  return (
    <Canvas
      mode='concurrent'
      style={{
        position: 'absolute',
        top: 0,
        backgroundColor: 'black',
      }}
      camera={{ fov: 75, near: 0.1, far: 1000, position: [0, 0, 20] }}
      onCreated={(state) => state.events.connect(dom.current)}
    >
      <LControl />
      <Preload all />
      {children}
      <EffectComposer multisampling={8}>
        <Bloom
          kernelSize={KernelSize.MEDIUM}
          luminanceThreshold={0}
          luminanceSmoothing={1}
          intensity={2}
          width={Resizer.AUTO_SIZE}
          height={Resizer.AUTO_SIZE}
        />
      </EffectComposer>
      <directionalLight position={[0, 0, 1]} />
      <ambientLight />
    </Canvas>
  )
}

export default LCanvas
