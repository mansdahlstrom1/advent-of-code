// const input = "1,9,10,3,2,3,11,0,99,30,40,50";
import fs from 'fs';
import path from 'path';

const input = fs.readFileSync(path.join(__dirname, '..', 'input', 'input.txt')).toString();
const array = input.split(",").map((i) => Number(i));

// Before running the program,
// replace position 1 with the value 12 and
array[1] = 12;
// replace position 2 with the value 2.
array[2] = 2;

for (let i = 0; i <= array.length; i += 4) {
  const optCode = array[i];
  const inputPos1 = array[i + 1];
  const inputPos2 = array[i + 2];
  const outputPos = array[i + 3];

  const value1 = array[inputPos1];
  const value2 = array[inputPos2];

  if (optCode === 99) {
    console.log("Got optCode 99; CLOSING PROGRAM!!!");
    break;
  } else if (optCode === 1) {
    array[outputPos] = value1 + value2;
  } else if (optCode === 2) {
    array[outputPos] = value1 * value2;
  }
}

// What value is left at position 0 after the program halts?
console.log(array[0]);
