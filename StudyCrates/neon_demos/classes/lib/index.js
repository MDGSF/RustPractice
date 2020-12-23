var addon = require('../native');

console.log(addon.hello());

const User = addon.User;

const oneUser = new User(0, 'Huang', 'Jian', '1342042894@qq.com');

console.log(oneUser);
console.log(oneUser.get("id"));
console.log(oneUser.get("first_name"));
console.log(oneUser.get("last_name"));
console.log(oneUser.get("email"));

