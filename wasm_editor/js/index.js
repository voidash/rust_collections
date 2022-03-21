
const cellSize = 50;
function draw(image) {
  const canvas = document.getElementById("my-canvas");
  const context = canvas.getContext("2d");


  //context.fillStyle="red";
  //context.fillRect(0,0,50,50);

  context.strokeStyle = "black";
  context.lineWidth = 1;

  const width = image.width();
  const height = image.height();

  const cells = image.cells();
  for(let x = 0 ; x< width; x++){
    for(let y = 0 ; y< width; y++){
       const index = ((y*width) + x)  * 3;
       const color = `rgb(${cells[index+0]}, ${cells[index+1]}, ${cells[index+2]})`
       context.fillStyle = color;
      context.fillRect(x * cellSize, y * cellSize, cellSize, cellSize);
    }
  }

  for (let x = 0 ; x < width ; x++) {
    context.beginPath();
    context.moveTo(x * cellSize, 0);
    context.lineTo(x*cellSize, height * cellSize);
    context.stroke();
  }

  for (let y = 0 ; y < height ; y++) {
    context.beginPath();
    context.moveTo(0,y * cellSize);
    context.lineTo(width*cellSize, y * cellSize);
    context.stroke();
  }
}


function setupCanvas(image) {
  const canvas = document.getElementById('my-canvas');
  canvas.addEventListener('click', (event) => {
    const rect = canvas.getBoundingClientRect();

    let x = event.clientX - rect.left;
    let y = event.clientY - rect.top;

    x = Math.floor(x / cellSize);
    y = Math.floor(y / cellSize);
    console.log(x, y);
    image.brush(x, y , [200, 255,200]);
    image.brush(x, y , [200, 255,200]);
    draw(image);

  });
}

async function main() {
  const lib = await import("../pkg/index.js").catch(console.error);
  const image = new lib.Image(10,10);

  setupCanvas(image);
  draw(image);
}

main();

