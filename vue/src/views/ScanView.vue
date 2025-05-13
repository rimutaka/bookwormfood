<template>
  <div>
    <div>
      <canvas id="canvas" class="scanCanvas" :width="CANVAS_SIZE.WIDTH" :height="CANVAS_SIZE.HEIGHT" />
    </div>
    <div className="scanBtn">
      <button @click.prevent="onBtnClickHandler" :class="btnClass">{{ btnText }}</button>
    </div>
  </div>
</template>

<!-- GitHub Copilot
1 vulnerability
client-side-unvalidated-url-redirection Allowing unvalidated redirection based on user-specified URLs -->

<script setup lang="ts">
import { ref, watchEffect, watch, onMounted, onBeforeUnmount } from 'vue';
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import router from '@/router';

const props = defineProps({
  decode: { type: Boolean, default: true },
  scanRate: { type: Number, default: 250 }
})

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



const store = useMainStore();

const btnText = ref(BTN_TXT.START)
const btnClass = ref('')
const scanning = ref(false)
const video = ref(document.createElement('video'))

let qrworker: Worker | null = null
let canvasElement: HTMLCanvasElement | null = null
let canvas: CanvasRenderingContext2D | null = null
let oldTime = 0

video.value.onplaying = () => {
  sx = (video.value.videoWidth - CANVAS_SIZE.WIDTH) / 2
  sy = (video.value.videoHeight - CANVAS_SIZE.HEIGHT) / 2
}

function initWorker() {
  qrworker = new Worker("wasmWorker.js")
  qrworker.onmessage = async ev => {
    if (qrworker && ev.data != null) {
      qrworker.terminate()
      const result = ev.data
      await stopScan()
      let res = result.data
      router.replace({ path: `/${res}` })
    }
  }
}

async function startScan() {
  initWorker()
  canvasElement = <HTMLCanvasElement | null>document.getElementById("canvas");

  if (!canvasElement) {
    console.error("Canvas element not found")
    return
  }

  canvas = <CanvasRenderingContext2D | null>canvasElement.getContext("2d", { willReadFrequently: true })

  btnText.value = BTN_TXT.STOP
  btnClass.value = "active"

  try {
    video.value.srcObject = await navigator.mediaDevices.getUserMedia(CAPTURE_OPTIONS)
    video.value.setAttribute("playsinline", "true")
    await video.value.play()
    scanning.value = true

    requestAnimationFrame(tick)
  } catch (err) {
    console.log("failed to start scan")
    stopScan()
    console.error(err)
  }
}

function stopScan() {
  scanning.value = false
  btnText.value = BTN_TXT.START
  btnClass.value = ""
  video.value.pause()
  if (video.value.srcObject) {
    (video.value.srcObject as MediaStream).getVideoTracks().forEach(track => {
      track.stop()
    })
    video.value.srcObject = null
  }
  router.replace({ path: "/" })
}

function tick(time: number) {

  if (!canvas || !video.value) {
    console.error("Canvas or video element not initialized")
    return
  }

  if (video.value.readyState === video.value.HAVE_ENOUGH_DATA) {
    canvas.drawImage(video.value, sx, sy, sw, sh, dx, dy, dw, dh)
    drawCrosshair()
    if (scanning.value) requestAnimationFrame(tick)
    if (props.decode) recogniseQRcode(time)
  }
  requestAnimationFrame(tick)
}

function drawCrosshair() {
  if (!canvas) {
    console.error("Canvas not initialized")
    return
  }

  canvas.fillStyle = "rgba(255,255,255,0.4)"
  const shape = new Path2D(crossHairSvg)
  canvas.fill(shape)
}

function recogniseQRcode(time: number) {
  if (!canvas) {
    console.error("Canvas not initialized")
    return
  }
  if (!qrworker) {
    console.error("QR worker not initialized")
    return
  }

  if (time - oldTime > props.scanRate) {
    oldTime = time
    let imageData = canvas.getImageData(x0, y0, crossHairWidth, crossHairHeight)
    qrworker.postMessage({ width: imageData.width, height: imageData.height })
    qrworker.postMessage(imageData, [imageData.data.buffer])
  }
}

async function onBtnClickHandler() {
  if (scanning.value) await stopScan()
  else await startScan()
}

// WASM init code as-is (if any) can be placed here

watchEffect(() => {
  document.title = "Book barcode scanner"
})

onMounted(() => {
  startScan().catch(console.error)
})
</script>