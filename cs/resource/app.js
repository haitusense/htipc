drawFromMemoryMap = async function(id, color, bitshift) {
  console.log("draw start")
  const canvas = document.getElementById(id, color);
  const ctx = canvas.getContext('2d', { willReadFrequently: true, alpha: false });
  const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);

  const src = await chrome.webview.hostObjects.SimpleGuiMmf.ReadPixelsForJS();
  // const clamp = new Uint8ClampedArray(dst);
  imageData.data.set(src);  
  ctx.putImageData(imageData, 0, 0);
  console.log("draw end")
  // canvas.width = 320;
  // canvas.height = 240;
}