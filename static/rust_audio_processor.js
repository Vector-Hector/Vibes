class MasterProcessor extends AudioWorkletProcessor {
    constructor() {
        super();
        this.buffers = [null, null];
        this.currentBuffer = 0;
        this.sampleIndex = 0;
        this.port.onmessage = this.handleMessage.bind(this);
    }

    handleMessage(event) {
        if (event.data.type === 'samples') {
            this.buffers[event.data.buffer] = event.data.samples;
        }
    }

    process(inputs, outputs) {
        const output = outputs[0];
        const currentSamples = this.buffers[this.currentBuffer];

        if (!currentSamples) return true;

        for (let channel = 0; channel < output.length; ++channel) {
            const outputChannel = output[channel];

            for (let i = 0; i < outputChannel.length; ++i) {
                outputChannel[i] = currentSamples[this.sampleIndex];

                this.sampleIndex++;
                if (this.sampleIndex >= currentSamples.length) {
                    // Request the next buffer from the worker
                    this.port.postMessage({ type: 'calculateSamples', length: currentSamples.length });
                    this.sampleIndex = 0;
                    this.currentBuffer = (this.currentBuffer + 1) % 2;
                }
            }
        }

        return true;
    }
}

registerProcessor('master-processor', MasterProcessor);
