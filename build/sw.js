if ("function" === typeof importScripts) {
  importScripts("https://storage.googleapis.com/workbox-cdn/releases/6.2.2/workbox-sw.js");
  if (workbox) {
    workbox.setConfig({debug: false});
    self.addEventListener("install", event => {
      self.skipWaiting();
    });
    workbox.precaching.precacheAndRoute([{"revision":"4fe99342398c285d43bd0781e52865c0","url":"asset-manifest.json"},{"revision":"4020571efe44dc33d271798e6a18e0c1","url":"favicon.ico"},{"revision":"f98923403f5c78d689d56b18f947c520","url":"img/apple-touch-icon.png"},{"revision":"a62aa63bb4d0a3dd08820787bd7e118f","url":"img/favicon-16x16.png"},{"revision":"a5229a03fcfe584a3031846fe3c19ccf","url":"img/favicon-32x32.png"},{"revision":"37ff8dc0d50cd7705fc65fc84837c2aa","url":"img/og-image-400.png"},{"revision":"ba20f5b6457b80105471f28899b1ffc7","url":"index.html"},{"revision":"2e93299ab8aad4f6859ba711fc9147c2","url":"static/css/main.788f695e.css"},{"revision":"d1de969ba433c3ada8f9686ea1407dfc","url":"static/js/main.1735d415.js"},{"revision":"3311ddb0ad85b9240262a753f5a667b7","url":"static/media/about.472d9c94914ce88e8d8f.svg"},{"revision":"8d00e8b89883db0a8a11d9f91b427d10","url":"static/media/borrow.f8356d8f6c1fc40fa23b.svg"},{"revision":"fb577b765b6c263191ff525af5f4f175","url":"static/media/buy.a0ebbd4b83f7c8afd5d9.svg"},{"revision":"ce7b3c19ccf67eeadbdf96d995c1cf6b","url":"static/media/isbn_mod_bg.3c792a329008a96c6665.wasm"},{"revision":"24f2b115d3964c9f977462cdd38b066a","url":"wasm/koder.js"},{"revision":"6f11e7db4fe9aca82cac7150bfc33769","url":"wasm/zbar.js"},{"revision":"e8789bf03df9c2c85e9c59ab0a0cd0c6","url":"wasm/zbar.wasm"},{"revision":"bb1c649a95ffa80369254cc3e51b9a41","url":"wasmWorker.js"}]);
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