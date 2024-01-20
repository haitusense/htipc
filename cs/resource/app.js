drawFromMemoryMap = async function(id, color, bitshift) {
  const canvas = document.getElementById(id, color);
  const ctx = canvas.getContext('2d', { willReadFrequently: true, alpha: false });
  const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);

  // const dst = await this.hostObj.Demosaicing(canvas.width, canvas.height, color, bitshift);
  let dst = Array(320 * 240 * 4);
  for (let y = 0; y < 240; y++) {
    for (let x = 0; x < 320; x++) {
      dst[(x + y * 320) * 4 + 0] = color;
      dst[(x + y * 320) * 4 + 1] = color;
      dst[(x + y * 320) * 4 + 2] = color;
      dst[(x + y * 320) * 4 + 3] = 255;
    }
  }

  const clamp = new Uint8ClampedArray(dst);
  imageData.data.set(clamp);  
  ctx.putImageData(imageData, 0, 0);
  console.log(color);
  // canvas.width = 320;
  // canvas.height = 240;
}