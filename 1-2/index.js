const fs = require('fs');
const input = fs.readFileSync('input').toString('utf8').split('').map(x => parseInt(x));
offset = input.length / 2;

console.log(input.reduce((acc, curr, index) => {
  const next = input[(index + offset) % input.length];
  return acc + (curr === next ? curr : 0);
}, 0));
