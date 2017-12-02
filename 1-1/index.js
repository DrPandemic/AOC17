const fs = require('fs');
input = fs.readFileSync('input').toString('utf8').split('');
input.push(input[0]);

console.log(input.reduce(([last, acc], curr) => {
  curr = parseInt(curr);
  return [curr, acc + (curr === last ? curr : 0)];
}, [-1, 0]));
