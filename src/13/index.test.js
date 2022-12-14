const { expect } = require('chai');
const { describe, it } = require('mocha');
const { comparePackets } = require('.');
const { getPairs } = require('./utils');

const pairs = getPairs('input/test.txt');

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
});
