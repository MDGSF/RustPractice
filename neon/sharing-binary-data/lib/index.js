var addon = require('../native');

console.log(addon.hello());

const uint8 = new Uint8Array([0, 1, 2, 3]);
var arrayBuffer = uint8.buffer; 
console.log(addon.binaryJsToRust(arrayBuffer));

console.log(addon.binaryRustToJs());

module.exports = require('../native/index.node');
