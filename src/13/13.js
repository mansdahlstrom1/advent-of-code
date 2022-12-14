const { getSumOfCorrectlyOrderedPackages } = require('.');
const { getPairs } = require('./utils');

const pairs = getPairs('input/input.txt');
const puzzelAnswer = getSumOfCorrectlyOrderedPackages(pairs);

console.log('=====\tDAY 13 - Puzzel Answer\t=====');
console.log(`=====\t\t ${puzzelAnswer}\t\t=====`);
console.log('=====\tDAY 13 - Puzzel Answer\t=====');
