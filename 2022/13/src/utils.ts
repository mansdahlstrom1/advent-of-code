import fs from 'fs';
import path from 'path';
import { NestedPair, Pair } from './types';

export const getPairs = (fileName: string): Pair[] => {
  const file = fs.readFileSync(path.join(__dirname, fileName)).toString();
  const pairs: Pair[] = file.split('\n\n').map((pair) => {
    const singlePair = pair.split('\n').map((row) => {
      if (row) {
        return JSON.parse(row) as NestedPair;
      }

      throw new Error('Invalid input, row was not truthy',);
    });

    if (singlePair.length !== 2) {
      throw new Error('Invalid input, singlePair was not length 2');
    }

    return singlePair
  });

  return pairs;
};


console.log('process.env.DEBUG', process.env.DEBUG);
export const log = (...args: any[]) => {
  if (process.env.DEBUG) {
    console.log(...args);
  }
}
