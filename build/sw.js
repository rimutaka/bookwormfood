if ("function" === typeof importScripts) {
  importScripts("https://storage.googleapis.com/workbox-cdn/releases/6.2.2/workbox-sw.js");
  if (workbox) {
    workbox.setConfig({debug: false});
    self.addEventListener("install", event => {
      self.skipWaiting();
    });
    workbox.precaching.precacheAndRoute([{"revision":"9ffa605687ecedd744aa38e552b5f54f","url":"asset-manifest.json"},{"revision":"4020571efe44dc33d271798e6a18e0c1","url":"favicon.ico"},{"revision":"f98923403f5c78d689d56b18f947c520","url":"img/apple-touch-icon.png"},{"revision":"a62aa63bb4d0a3dd08820787bd7e118f","url":"img/favicon-16x16.png"},{"revision":"a5229a03fcfe584a3031846fe3c19ccf","url":"img/favicon-32x32.png"},{"revision":"37ff8dc0d50cd7705fc65fc84837c2aa","url":"img/og-image-400.png"},{"revision":"085b7864d9041485e8b3f4444fbdd09a","url":"index-pocket.html"},{"revision":"362da31e10855834848960da99b842e9","url":"index.html"},{"revision":"0d1e6da8a5d82671c489f9b0e264ec5f","url":"static/css/main.02ddbbc4.css"},{"revision":"5efaa7fe29b712273f530fccb8a2f2f6","url":"static/js/main.f922ffaa.js"},{"revision":"2a50d2547727348b5e6ed593f8dcf7be","url":"static/media/isbn_mod_bg.5d4d316444e5847f6d91.wasm"},{"revision":"24f2b115d3964c9f977462cdd38b066a","url":"wasm/koder.js"},{"revision":"6f11e7db4fe9aca82cac7150bfc33769","url":"wasm/zbar.js"},{"revision":"e8789bf03df9c2c85e9c59ab0a0cd0c6","url":"wasm/zbar.wasm"},{"revision":"bb1c649a95ffa80369254cc3e51b9a41","url":"wasmWorker.js"}]);
    workbox.routing.registerRoute(
      new RegExp("https://fonts.(?:.googlepis|gstatic).com/(.*)"),
      new workbox.strategies.CacheFirst({
        cacheName: "googleapis",
        plugins: [
          new workbox.cacheableResponse.CacheableResponsePlugin({
            maxEntries: 30
          })
        ]
      })
    );
    workbox.routing.registerRoute(
      /\.(?:png|gif|jpg|jpeg|svg|ico)$/,
      new workbox.strategies.CacheFirst({
        cacheName: "images",
        plugins: [
          new workbox.cacheableResponse.CacheableResponsePlugin({
            maxEntries: 60,
            maxAgeSeconds: 30 * 24 * 60 * 60 // 30 Days
          })
        ]
      })
    );
    workbox.routing.registerRoute(
      /\.(?:js|css|wasm|json)$/,
      new workbox.strategies.StaleWhileRevalidate({
        cacheName: "static-resources",
        plugins: [
          new workbox.cacheableResponse.CacheableResponsePlugin({
            maxEntries: 60,
            maxAgeSeconds: 20 * 24 * 60 * 60 // 20 Days
          })
        ]
      })
    );
  } else console.error("Workbox could not be loaded. No offline support");
}