const drawFromMemoryMap = async function(id, bitshift) {
  const w = await chrome.webview.hostObjects.SimpleGuiMmf.Width();
  const h = await chrome.webview.hostObjects.SimpleGuiMmf.Height();

  console.log("drawing", w, h, bitshift)
  const canvas = document.getElementById(id);
  const ctx = canvas.getContext('2d', { willReadFrequently: true, alpha: false });
  const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);

  const src = await chrome.webview.hostObjects.SimpleGuiMmf.ReadPixelsForJS(bitshift);
  // const clamp = new Uint8ClampedArray(dst);
  imageData.data.set(src);
  ctx.putImageData(imageData, 0, 0);
  // canvas.width = 320;
  // canvas.height = 240;
}

const dispatch = (json) => { 
  // chrome.webview.hostObjects.hoge.dispatcher(e);
  window.chrome.webview.postMessage(json); 
}

/* actionCreator */

setUserName = async (e) => await chrome.webview.hostObjects.actionCreator.setUserName(e);
setUser2 = async (e) => JSON.stringify({ type : "test", payload: e });
