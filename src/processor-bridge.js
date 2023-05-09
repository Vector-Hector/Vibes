let masterProcessor = null;
let audioContext = null;

function resolveAfter(s) {
    return new Promise((resolve) => {
        setTimeout(() => {
            resolve(null);
        }, s * 1000);
    });
}

async function createMaster() {
    if (audioContext !== null)
        return;

    console.log("start create master")

    audioContext = new AudioContext();
    await audioContext.audioWorklet.addModule('static/rust_audio_processor.js');

    const gainNode = new GainNode(audioContext);
    gainNode.gain.value = 0.1;
    gainNode.connect(audioContext.destination);

    console.log("creating master")

    masterProcessor = new AudioWorkletNode(audioContext, 'master-processor');
    masterProcessor.connect(gainNode);

}

export async function setWorker(worker) {
    await createMaster();

    while (!masterProcessor) {
        await resolveAfter(0.1);
    }

    masterProcessor.port.onmessage = (event) => {
        worker.postMessage(event.data);
    }
}

export async function setOnMessage(handler) {
    await createMaster();

    masterProcessor.port.onmessage = handler;
}

export async function initialize() {
    await createMaster();
}

export async function sendSamples(samples, buffer) {
    if (masterProcessor === null) {
        await createMaster();
    }

    masterProcessor.port.postMessage({ type: 'samples', samples: samples, buffer: buffer });
}

export async function play() {
    if (masterProcessor === null) {
        await createMaster();
    }

    audioContext.resume();

}
