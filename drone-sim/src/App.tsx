import { RenderObject, Simulation } from "sim-engine";
import { useEffect, useRef, useState } from "react";
import Grid from "./grid";
import Header from "./header";


function App() {
  const [height_miles, setHeight] =useState(5);
  const [width_miles, setWidth] =useState(5);
  const [milePerUnit, setMilePerUnit] =useState(0.007);
  const [objects, setObjects] = useState<RenderObject[]>([]);
  const simRef = useRef<Simulation| null>(null);

  useEffect(()=>{
    const sim = new Simulation(height_miles, width_miles);
    simRef.current = sim;
    const id = sim.add_vehicle("drone", sim.width/2, sim.height/2, 5, 0, 40, 20, 8);
    sim.add_go_to_location(id, 1, 1, 1);
    // function updateLoop(){
    //   setObjects(sim.tick());  
    //   requestAnimationFrame(updateLoop);
    // }
    // requestAnimationFrame(updateLoop);
  }, [])

  function testTick(){
    if (!simRef.current){return;}
    setObjects(simRef.current.tick())
  }
  
  return (
    <div>
      <Header/>
      <Grid objects={objects} height_miles={height_miles} width_miles={width_miles} milePerUnit={milePerUnit}/>
      <button onClick={testTick}>Tick</button>
    </div>
  );
}

export default App;