const {
  getSumOfCorrectlyOrderedPackages,
  followDistressSignalProtocol,
  sortPackages,
  findDecoderKey,
} = require('.');
const { getPairs } = require('./utils');

const pairs = getPairs('input/input.txt');
const puzzelAnswer = getSumOfCorrectlyOrderedPackages(pairs);

console.log('=====\tDAY 13 - pt 1\t=====');
console.log(`=====\t\t ${puzzelAnswer}\t\t=====`);
console.log('=====\tDAY 13 - pt 1\t=====');

const flat = followDistressSignalProtocol(pairs);
const sorted = flat.sort(sortPackages);
const decoderKey = findDecoderKey(sorted);

console.log('=====\tDAY 13 - pt 2\t=====');
console.log(`=====\t${decoderKey}\t\t=====`);
console.log('=====\tDAY 13 - pt 2\t=====');
