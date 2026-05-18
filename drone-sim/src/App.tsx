/* eslint-disable @typescript-eslint/no-unused-vars */
import { RenderObject, Simulation } from "sim-engine";
import { useEffect, useRef, useState } from "react";
import Grid from "./grid";
import Header from "./header";


function App() {
  const [height_miles, _setHeight] =useState(5);
  const [width_miles, _setWidth] =useState(5);
  const [milePerUnit, _setMilePerUnit] =useState(0.007);
  const [objects, setObjects] = useState<RenderObject[]>([]);
  const simRef = useRef<Simulation| null>(null);
  const pausedRef = useRef(true);
  const [paused, setPaused] = useState(true);

  useEffect(()=>{
    const sim = new Simulation(height_miles, width_miles);
    simRef.current = sim;
    const id = sim.add_vehicle("drone", 20, width_miles/2, height_miles/2, 1, 90, 40, 20, 8);
    sim.add_go_to_location(id, 1, 1, 1);

    function updateLoop(){
      if(!pausedRef.current){
        setObjects(sim.tick());
      }  
      requestAnimationFrame(updateLoop);
    }
    
    requestAnimationFrame(updateLoop);
  }, [])

  useEffect(()=>{
    pausedRef.current = paused;
  }, [paused]);

  function pauseButton(){
    setPaused(!paused);
  }
  
  return (
    <div>
      <Header/>
      <Grid objects={objects} height_miles={height_miles} width_miles={width_miles} milePerUnit={milePerUnit}/>
      <button onClick={pauseButton}>{paused ? "Play": "Pause"}</button>
    </div>
  );
}

export default App;