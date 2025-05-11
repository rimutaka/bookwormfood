importScripts("wasm/zbar.js");
importScripts("wasm/koder.js");


(async () => {
  // Initialize Koder
  const koder = await new Koder().initialize({ wasmDirectory: "./wasm" });

  // console.log("Koder initialised");

  // Listen for messages from JS main thread containing raw image data
  self.addEventListener('message', event => {
    if ('width' in event.data && 'height' in event.data) {
      this.width = event.data.width;
      this.height = event.data.height;
    }

    const { data } = event.data;
    if (!data) return;

    const t0 = new Date().getTime();
    // const scanResult = { "code": "9781761186769", "type": "EAN-13" }; // Everything is Beautiful and Everything Hurts
    // const scanResult = {"code":"9781529109474","type":"EAN-13"}; // Ottolenghi Test Kitchen: Extra Good Things 
    // const scanResult = {"code":"9780998558547","type":"EAN-13"}; // Animal Line Drawing 
    // const scanResult = {"code":"9780998558111","type":"EAN-13"}; // Non-existent ISBN
    const scanResult = koder.decode(data, this.width, this.height);
    const t1 = new Date().getTime();
    if (scanResult) {
      // console.log(JSON.stringify(scanResult)); // {"code":"9781465483942","type":"EAN-13"}

      // only rack ISBN codes - should be limited in the reader, but that's for later
      // https://en.wikipedia.org/wiki/ISBN
      if (!scanResult.code || !scanResult.code.startsWith("97")) {
        // console.log(scanResult.code);
        return;
      }
      postMessage({
        data: scanResult.code,
        type: scanResult.type,
        ms: t1 - t0
      });
    }
  });
})();