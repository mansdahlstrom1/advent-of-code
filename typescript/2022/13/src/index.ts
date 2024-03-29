/**
 * If both values are integers, the lower integer should come first.
 * If the left integer is lower than the right integer, the inputs are in the right order.
 * If the left integer is higher than the right integer, the inputs are not in the right order.
 * Otherwise, the inputs are the same integer; continue checking the next part of the input.
 *
 * If both values are lists, compare the first value of each list, then the second value, and so on.
 * If the left list runs out of items first, the inputs are in the right order.
 * If the right list runs out of items first, the inputs are not in the right order.
 * If the lists are the same length and no comparison makes a decision about the order,
 * continue checking the next part of the input.
 *
 * If exactly one value is an integer, convert the integer to a list which contains that integer as its only value, then retry the comparison.
 * For example, if comparing [0,0,0] and 2, convert the right value to [2] (a list containing 2);
 * the result is then found by instead comparing [0,0,0] and [2].
 */

import { NestedPair, Pair } from "./types";
import { log } from "./utils";

export const getSumOfCorrectlyOrderedPackages = (pairs: Pair[]) => {
  let sum = 0;

  for (let [index, pair] of pairs.entries()) {
    const value = comparePackets(pair);
    if (value) {
      sum += index + 1;
    }
  }
  return sum;
};

export const comparePackets = (pair: Pair): boolean | undefined => {
  const [left, right] = pair;

  for (let [index, leftValue] of left.entries()) {
    const rightValue = right[index];

    /** If the right list runs out of items first, the inputs are not in the right order. */
    if (rightValue === undefined) {
      log('Right side ran out of items, so inputs are not in the right order');
      return false;
    }
    /** If both values are integers, compare Integers */
    if (Number.isInteger(leftValue) && Number.isInteger(rightValue)) {
      if (leftValue === rightValue) continue;

      if (leftValue < rightValue) {
        log('Left side is smaller, so inputs are in the right order');
        return true;
      }

      log('Right side is smaller, so inputs are not in the right order');
      return false;
    }

    /** If both values are lists, compare the first value of each list, then the second value, and so on. */
    if (Array.isArray(leftValue) && Array.isArray(rightValue)) {
      const comparison = comparePackets([leftValue, rightValue]);
      if (comparison !== undefined) {
        return comparison;
      }
    }

    /**
     * If exactly one value is an integer, convert the integer to a list which contains that integer as its only value, then retry the comparison.
     * For example, if comparing [0,0,0] and 2, convert the right value to [2] (a list containing 2);
     * the result is then found by instead comparing [0,0,0] and [2].
     */
    if (typeof leftValue === 'number' && typeof rightValue !== 'number') {
      const comparison = comparePackets([[leftValue], rightValue]);
      if (comparison !== undefined) {
        return comparison;
      }
    } else if (typeof leftValue !== 'number' && typeof rightValue === 'number') {
      const comparison = comparePackets([leftValue, [rightValue]]);
      if (comparison !== undefined) {
        return comparison;
      }
    }
    log('end of the line');
  }

  /** If the left list runs out of items first, the inputs are in the right order. */
  if (left.length < right.length) {
    log('Left side ran out of items, so inputs are in the right order');
    return true;
  }
};

export const sortPackages = (a: NestedPair, b: NestedPair) => {
  const value = comparePackets([b, a]);
  return value ? 1 : -1;
};

/**
 * The distress signal protocol also requires that you include two additional divider packets:
 *
 * ```
 * [[2]]
 * [[6]]
 * ```
 *
 * @param {*} pairs pairs of packets
 * @returns a flat list of all packets
 */
export const followDistressSignalProtocol = (pairs: Pair[]) => {
  const flat = pairs.reduce((acc, val) => [...acc, ...val], []);

  // push the new values in
  flat.push([[2]]);
  flat.push([[6]]);
  return flat;
};

export const findDecoderKey = (sortedList: NestedPair): number => {
  const indices: number[] = [];
  sortedList.forEach((row, i) => {
    const stringRow = JSON.stringify(row);
    if (stringRow === '[[2]]' || stringRow === '[[6]]') {
      indices.push(i + 1);
    }
  });

  return indices.reduce((acc, val) => acc * val, 1);
};
