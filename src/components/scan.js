import React, { useEffect } from "react";
import useState from 'react-usestateref';
import { useNavigate } from "react-router-dom";

const BTN_TXT = {
  START: "SCAN ISBN",
  STOP: "STOP SCANNING"
};

const CANVAS_SIZE = {
  WIDTH: 320,
  HEIGHT: 430
};

const CAPTURE_OPTIONS = {
  audio: false,
  video: { facingMode: "environment" }
}

const sw = CANVAS_SIZE.WIDTH;
const sh = CANVAS_SIZE.HEIGHT;
const dw = sw;
const dh = sh;
const dx = 0;
const dy = 0;
let sx = 0;
let sy = 0;

const crossHairSvg = "M77.125 148.02567c0-3.5774 2.73862-6.27567 6.37076-6.27567H119V117H84.0192C66.50812 117 52 130.77595 52 148.02567V183h25.125v-34.97433zM237.37338 117H202v24.75h35.18494c3.63161 0 6.69006 2.69775 6.69006 6.27567V183H269v-34.97433C269 130.77595 254.88446 117 237.37338 117zM243.875 285.4587c0 3.5774-2.73863 6.27567-6.37076 6.27567H202V317h35.50424C255.01532 317 269 302.70842 269 285.4587V251h-25.125v34.4587zM83.49576 291.73438c-3.63213 0-6.37076-2.69776-6.37076-6.27568V251H52v34.4587C52 302.70842 66.50812 317 84.0192 317H119v-25.26563H83.49576z";
const crossHairWidth = 217, crossHairHeight = 200, x0 = 53, y0 = 117;

export default function Scan({
  decode = true,
  scanRate = 250,
}) {

  // Component state
  const [btnText, setBtnText] = useState(BTN_TXT.START);
  const [btnClass, setBtnClass] = useState();
  const [scanning, setScanning] = useState(false);

  const [video] = useState(document.createElement("video"));

  const navigate = useNavigate();

  // Constants
  let qrworker = null;
  let canvasElement = null;
  let canvas = null;
  let oldTime = 0;

  video.onplaying = () => {
    sx = (video.videoWidth - CANVAS_SIZE.WIDTH) / 2;
    sy = (video.videoHeight - CANVAS_SIZE.HEIGHT) / 2;
  };

  const initWorker = () => {
    qrworker = new Worker("wasmWorker.js");
    qrworker.onmessage = async ev => {
      if (ev.data != null) {
        qrworker.terminate();
        const result = ev.data;
        await stopScan();

        let res = result.data;

        navigate(`/${res}`);
      }
    };
  };

  const startScan = async () => {
    initWorker();
    canvasElement = document.getElementById("canvas");
    canvas = canvasElement.getContext("2d", { willReadFrequently: true });

    setBtnText(BTN_TXT.STOP);
    setBtnClass("active");

    try {
      video.srcObject = await navigator.mediaDevices.getUserMedia(CAPTURE_OPTIONS);
      video.setAttribute("playsinline", "true");
      await video.play();
      setScanning(true);

      requestAnimationFrame(tick);
    } catch (err) {
      console.log("failed to start scan");
      stopScan();
      console.error(err);
    }
  };

  const stopScan = () => {
    setScanning(false);
    setBtnText(BTN_TXT.START);
    setBtnClass("");
    video.pause();
    if (video.srcObject) {
      video.srcObject.getVideoTracks().forEach(track => {
        track.stop();
      });
      video.srcObject = null;
    }
  };

  const tick = (time) => {
    if (video.readyState === video.HAVE_ENOUGH_DATA) {
      canvas.drawImage(video, sx, sy, sw, sh, dx, dy, dw, dh);

      drawCrosshair();
      if (scanning) requestAnimationFrame(tick);
      if (decode) recogniseQRcode(time);
    }
    requestAnimationFrame(tick);
  };


  const drawCrosshair = () => {
    canvas.fillStyle = "rgba(255,255,255,0.4)";
    const shape = new Path2D(crossHairSvg);
    canvas.fill(shape);
  };

  const recogniseQRcode = (time) => {
    if (time - oldTime > scanRate) {
      oldTime = time;
      let imageData;
      imageData = canvas.getImageData(x0, y0, crossHairWidth, crossHairHeight);
      qrworker.postMessage({ width: imageData.width, height: imageData.height });
      qrworker.postMessage(imageData, [imageData.data.buffer]);
    }
  };

  const onBtnClickHandler = async (e) => {
    e.preventDefault();
    if (scanning) await stopScan(); else await startScan();
  };


  useEffect(() => {

    // these values are used to set the meta tags in index.html
    // and have to be reset when the component is mounted from
    // a scan that sets them to the book details
    // make sure the values are synchronized with index.html
    // TODO: change ids to constants
    document.title = "Book barcode scanner"
    document.getElementById("ogImage").setAttribute('content', "Scan book barcodes to record or share the books");
    document.getElementById("ogTitle").setAttribute('content', "/logo.png");

    const startScanOnce = async () => {
      await startScan();
    }
    startScanOnce().catch(console.error);
  }, []);

  return (
    <div>
      <div>
        <canvas id="canvas" className="scanCanvas" width={CANVAS_SIZE.WIDTH} height={CANVAS_SIZE.HEIGHT} />
      </div>
      <div className="scanBtn">
        <button onClick={onBtnClickHandler} className={btnClass}>{btnText}</button>
      </div>
    </div>
  )
};
