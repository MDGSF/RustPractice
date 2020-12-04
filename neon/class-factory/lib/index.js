var addon = require('../native');

console.log(addon.hello());

var LogFactory = addon.LogFactory;

const factory = new LogFactory();

function run() {
  const loggerA = factory.create('A');

  loggerA.log('This is A');

  const loggerB = factory.create('B');

  loggerB.log('This is B');
  loggerA.log('This is A');
}

run();

if (typeof global.gc === 'function') {
  global.gc();
}

factory.create('C').log('This is C');
factory.lazy('D').log('This is D');
