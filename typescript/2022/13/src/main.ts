import {
  getSumOfCorrectlyOrderedPackages,
  followDistressSignalProtocol,
  sortPackages,
  findDecoderKey,
} from '.';

import { getPairs } from './utils';

const pairs = getPairs('../input/input.txt');
const puzzleAnswer = getSumOfCorrectlyOrderedPackages(pairs);

console.log('=====\tDAY 13 - pt 1\t=====');
console.log(`=====\t${puzzleAnswer}\t\t=====`);
console.log('=====\tDAY 13 - pt 1\t=====\n');

const flat = followDistressSignalProtocol(pairs);
const sorted = flat.sort(sortPackages);
const decoderKey = findDecoderKey(sorted);

console.log('=====\tDAY 13 - pt 2\t=====');
console.log(`=====\t${decoderKey}\t\t=====`);
console.log('=====\tDAY 13 - pt 2\t=====');
