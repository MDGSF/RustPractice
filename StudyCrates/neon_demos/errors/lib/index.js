var addon = require('../native');

console.log(addon.hello());

addon.throwError("a");
