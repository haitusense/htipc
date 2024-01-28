drawFromMemoryMap = async function(id, bitshift) {
  const w = await chrome.webview.hostObjects.SimpleGuiMmf.Width();
  const h = await chrome.webview.hostObjects.SimpleGuiMmf.Height();


  console.log("drawing", w, h, bitshift)
  const canvas = document.getElementById(id);
  const ctx = canvas.getContext('2d', { willReadFrequently: true, alpha: false });
  const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);

  const src = await chrome.webview.hostObjects.SimpleGuiMmf.ReadPixelsForJS(bitshift);
  console.log(src)
  // const clamp = new Uint8ClampedArray(dst);
  imageData.data.set(src);
  ctx.putImageData(imageData, 0, 0);
  // canvas.width = 320;
  // canvas.height = 240;
}

action = function(json) { window.chrome.webview.postMessage(json) }
