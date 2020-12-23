var addon = require('../native');

console.log(addon.hello());

console.log(addon.convertVecToArray());

console.log(addon.convertJsArrayToVec([1, 2, 3]));

console.log(addon.returnJsArray());

console.log(addon.returnJsArrayWithNumber());

console.log(addon.returnJsArrayWithString());

module.exports = require('../native/index.node');
