import  * as td from "./text_decoder.js";
import * as worker from "./worker/audio_worker.js"

const _ = td;

class MasterProcessor extends AudioWorkletProcessor {
    constructor() {
        super();
        this.port.onmessage = this.handleMessage.bind(this);
    }

    async handleMessage(event) {
        switch (event.data.type) {
            case "midi":
                console.log("processor got midi message", event.data)
                const msg = event.data;
                worker.on_midi(msg.is_active, msg.note, msg.velocity);
                break;
            case "wasmModule":
                const module = event.data.value;
                worker.initSync(module);
                console.log("initialized wasm module")
                break;
            case "waveTable":
                const waveTable = event.data.value;
                console.log("wave table:", waveTable);
                worker.set_wave_table(waveTable);
                console.log("set wave table")
                break;
        }
    }

    process(inputs, outputs) {
        const output = outputs[0];
        const currentSamples = worker.calculate_samples(output[0].length);

        for (let channel = 0; channel < output.length; ++channel) {
            const outputChannel = output[channel];

            for (let i = 0; i < outputChannel.length; ++i) {
                outputChannel[i] = currentSamples[i];
            }
        }

        return true;
    }
}

registerProcessor('master-processor', MasterProcessor);

