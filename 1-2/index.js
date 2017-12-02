const fs = require('fs');
input = fs.readFileSync('input').toString('utf8').split('');
offset = input.length / 2;

console.log(input.reduce((acc, curr, index) => {
  curr = parseInt(curr);
  const next = parseInt(input[(index + offset) % input.length]);
  return acc + (curr === next ? curr : 0);
}, 0));
