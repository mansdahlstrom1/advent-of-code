import { expect } from 'chai';
import { describe, it } from 'mocha';
import { comparePackets, sortPackages, followDistressSignalProtocol, findDecoderKey } from '.';
import { getPairs } from './utils';

const pairs = getPairs('../input/test.txt');

describe('13', () => {
  it('Should handle Pair 1', () => {
    console.log('==== PAIR 1 =====');

    const isInOrder = comparePackets(pairs[0]);
    expect(isInOrder).to.eql(true);
  });

  it('Should handle Pair 2', () => {
    console.log('==== PAIR 2 =====');

    const isInOrder = comparePackets(pairs[1]);
    expect(isInOrder).to.eql(true);
  });

  it('Should handle Pair 3', () => {
    console.log('==== PAIR 3 =====');

    const isInOrder = comparePackets(pairs[2]);
    expect(isInOrder).to.eql(false);
  });

  it('Should handle Pair 4', () => {
    console.log('==== PAIR 4 =====');

    const isInOrder = comparePackets(pairs[3]);
    expect(isInOrder).to.eql(true);
  });

  it('Should handle Pair 5', () => {
    console.log('==== PAIR 5 =====');

    const isInOrder = comparePackets(pairs[4]);
    expect(isInOrder).to.eql(false);
  });

  it('Should handle Pair 6', () => {
    console.log('==== PAIR 6 =====');

    const isInOrder = comparePackets(pairs[5]);
    expect(isInOrder).to.eql(true);
  });

  it('Should handle Pair 7', () => {
    console.log('==== PAIR 7 =====');

    const isInOrder = comparePackets(pairs[6]);
    expect(isInOrder).to.eql(false);
  });

  it('Should handle Pair 8', () => {
    console.log('==== PAIR 8 =====');

    const isInOrder = comparePackets(pairs[7]);
    expect(isInOrder).to.eql(false);
  });

  it('should sort the packets', () => {
    const flat = followDistressSignalProtocol(pairs);
    const sorted = flat.sort(sortPackages);

    expect(sorted[0]).to.eql([]);
    expect(sorted[1]).to.eql([[]]);
    expect(sorted[2]).to.eql([[[]]]);
    expect(sorted[3]).to.eql([1, 1, 3, 1, 1]);
    expect(sorted[4]).to.eql([1, 1, 5, 1, 1]);
    expect(sorted[5]).to.eql([[1], [2, 3, 4]]);
    expect(sorted[6]).to.eql([1, [2, [3, [4, [5, 6, 0]]]], 8, 9]);
    expect(sorted[7]).to.eql([1, [2, [3, [4, [5, 6, 7]]]], 8, 9]);
    expect(sorted[8]).to.eql([[1], 4]);
    expect(sorted[9]).to.eql([[2]]);
    expect(sorted[10]).to.eql([3]);
    expect(sorted[11]).to.eql([[4, 4], 4, 4]);
    expect(sorted[12]).to.eql([[4, 4], 4, 4, 4]);
    expect(sorted[13]).to.eql([[6]]);
    expect(sorted[14]).to.eql([7, 7, 7]);
    expect(sorted[15]).to.eql([7, 7, 7, 7]);
    expect(sorted[16]).to.eql([[8, 7, 6]]);
    expect(sorted[17]).to.eql([9]);

    const decoderKey = findDecoderKey(sorted);
    expect(decoderKey).to.eql(140);
  });
});
