import { Simulation } from "sim-engine";
import { useEffect, useRef, useState } from "react";
import Grid from "./grid";
import Header from "./header";


function App() {
  const [height, setHeight] =useState(100);
  const [width, setWidth] =useState(150);
  const [milePerUnit, setMilePerUnit] =useState(0.15);
  const [objects, setObjects] = useState([]);
  const simRef = useRef<Simulation| null>(null);

  useEffect(()=>{
    const sim = new Simulation(height/milePerUnit, width/milePerUnit);
    simRef.current = sim;
    sim.add_vehicle("drone", width/2, height/2, 0, 40, 20, 8);
    function updateLoop(){
      setObjects(sim.tick());  
      requestAnimationFrame(updateLoop);
    }
    requestAnimationFrame(updateLoop);
  }, [])
  
  return (
    <div>
      <Header/>
      <Grid objects={objects} height={height} width={width} milePerUnit={milePerUnit}/>

    </div>
  );
}

export default App;