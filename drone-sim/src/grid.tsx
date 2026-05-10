import { useEffect, useRef } from "react";
import type { RenderObject } from "sim-engine";

type GridProps = {
    objects: RenderObject[],
    height: number;
    width: number;
    milePerUnit:number;
}
const visualScalerAmount = 500;
function drawBoundry(ctx: CanvasRenderingContext2D, canvasHeight: number, canvasWidth: number){
    ctx.beginPath()
    ctx.moveTo(0, 0);
    ctx.lineTo(0, canvasHeight);
    ctx.lineTo(canvasWidth, canvasHeight);
    ctx.lineTo(canvasWidth, 0);
    ctx.lineTo(0, 0);
    ctx.stroke();
}

function drawObjects(objects: RenderObject[], milePerUnit:number, ctx: CanvasRenderingContext2D){
  objects.forEach(object => {
    ctx.save()
    ctx.translate(object.x/milePerUnit, object.y/milePerUnit);
    ctx.rotate(object.heading);
    const width = (object.width_feet/5280/milePerUnit)*visualScalerAmount;
    const length = (object.length_feet/5280/milePerUnit)*visualScalerAmount;
    // const height = object.height_feet/5280;
    if(object.name === "drone"){
        ctx.fillStyle = "red";
        ctx.beginPath();

        ctx.moveTo(-width / 2, -length / 2);
        ctx.lineTo(width / 2, -length / 2);
        ctx.lineTo(0, length / 2);

        ctx.closePath();
        ctx.fill();
    } else if (object.name === "proj"){
        ctx.fillStyle = "black";
        ctx.beginPath();

        ctx.moveTo(-width/2, - length / 2);
        ctx.lineTo(width / 2, -length / 2);
        ctx.lineTo(width/2, length/2);
        ctx.lineTo(-width/2, length/2);
        ctx.lineTo(-width/2, -length/2);
        ctx.closePath();
        ctx.fill();
    }
    else {
        console.log("not matching anything:", object.name);
    }
    ctx.restore()
  });
}


function Grid({ objects, height, width, milePerUnit }: GridProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);


  useEffect(() => {
    const canvas = canvasRef.current!;
    const ctx = canvas.getContext("2d")!;

    canvas.height = height/milePerUnit;
    canvas.width = width/milePerUnit;

    ctx.clearRect(0, 0, canvas.width, canvas.height);

    ctx.strokeStyle = "#000000";
    drawBoundry(ctx, canvas.height, canvas.width);
    drawObjects(objects, milePerUnit, ctx);
  }, [objects, height, width, milePerUnit]);

  return <canvas ref={canvasRef} />;
}

export default Grid;