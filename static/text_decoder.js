let worker = null;

class TD {
    decode(data) {
        if (!data)
            return "";
        return this.utf8ArrayToString(data);
    }

    utf8ArrayToString(array) {
        let result = '';
        for (let i = 0; i < array.length; i++) {
            const byte = array[i];
            if (byte < 0x80) {
                result += String.fromCharCode(byte);
            } else if (byte >= 0xC0 && byte < 0xE0) {
                const byte2 = array[++i];
                result += String.fromCharCode(((byte & 0x1F) << 6) | (byte2 & 0x3F));
            } else if (byte >= 0xE0 && byte < 0xF0) {
                const byte2 = array[++i];
                const byte3 = array[++i];
                result += String.fromCharCode(
                    ((byte & 0x0F) << 12) | ((byte2 & 0x3F) << 6) | (byte3 & 0x3F)
                );
            }
        }
        return result;
    }
}

globalThis.TextDecoder = TD;

