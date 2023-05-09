import * as worker from "./worker/audio_worker.js"

let currentBuffer = 0;

const onMessage = (event) => {
    if (event.data.type !== "calculateSamples")
        return;

    const samples = worker.calculate_samples(event);

    console.log("sending samples", samples.length)

    self.postMessage({
        type: "samples",
        buffer: currentBuffer,
        samples: samples,
    })

    currentBuffer = (currentBuffer + 1) % 2;
};

worker.default("/static/worker/audio_worker_bg.wasm").then(() => {
    console.log("registering listener")

    self.addEventListener("message", onMessage)
});

