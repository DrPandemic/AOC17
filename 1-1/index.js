const fs = require('fs');
const input = fs.readFileSync('input').toString('utf8').split('').map(x => parseInt(x));
input.push(input[0]);

console.log(input.reduce(([last, acc], curr) => {
  return [curr, acc + (curr === last ? curr : 0)];
}, [-1, 0]));
