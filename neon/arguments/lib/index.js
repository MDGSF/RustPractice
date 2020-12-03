var addon = require('../native');

console.log(addon.hello());

console.log(addon.printFunction(() => {}));

console.log(addon.add1ToArgument(1));

console.log(addon.getArgsLen(1, 2));
console.log(addon.getArgsLen(1, 2, 3));
console.log(addon.getArgsLen(1, 2, 3, 4));

console.log(addon.argsOpt(123));

console.log(addon.defaultArgs());
console.log(addon.defaultArgs(1));
console.log(addon.defaultArgs(1000, "kaka"));

console.log(addon.acceptsJsArrays([1, 2, 3, 4]));

console.log(addon.acceptsJsObjects({"myProp": "Hello rust javascript"}));

module.exports = require('../native/index.node');
