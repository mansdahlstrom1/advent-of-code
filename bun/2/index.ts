import { readData } from "../utils";

const findNumberDuplicatesTwice = (ranges: string[]) => {
  const duplicates: number[] = [];

  ranges.map((range) => {
    const [start, end] = range.split("-");

    if (!start || !end) {
      throw new Error(`Missing start and end value for range: ${range}`);
    }

    for (let i = Number(start); i <= Number(end); i++) {
      const numberAsString = String(i);
      const len = numberAsString.length;
      const firstHalf = numberAsString.substring(0, len / 2);
      const secondHalf = numberAsString.substring(len / 2, len);

      if (firstHalf === secondHalf) {
        duplicates.push(i);
      }
    }
  });

  return duplicates;
};

const solvePt1 = (rows: string[]) => {
  const ranges = rows[0].split(",");
  const duplicates = findNumberDuplicatesTwice(ranges);
  console.log(duplicates);

  const totalExample = duplicates.reduce((acc, cur) => acc + cur, 0);
  console.log(totalExample);
};

const rowsExamplePt1 = await readData("2/input/example.txt");
solvePt1(rowsExamplePt1);

const rowsInputPt1 = await readData("2/input/input.txt");
solvePt1(rowsInputPt1);

const findDuplicates = (ranges: string[]) => {
  const duplicates: number[] = [];

  ranges.map((range) => {
    const [start, end] = range.split("-");

    if (!start || !end) {
      throw new Error(`Missing start and end value for range: ${range}`);
    }

    for (let i = Number(start); i <= Number(end); i++) {
      const numberAsString = String(i);
      const len = numberAsString.length;

      let subString = "";
      for (let y = 0; y < len; y++) {
        subString += numberAsString[y];
        const count = numberAsString.split(subString).length - 1;

        // Must be at least 2 instances and must cover the entire string
        if (count >= 2 && count * subString.length === len) {
          duplicates.push(i);
          break;
        }
      }
    }
  });

  return duplicates;
};

const solvePt2 = (rows: string[]) => {
  const ranges = rows[0].split(",");
  const duplicates = findDuplicates(ranges);
  console.log(duplicates);

  const totalExample = duplicates.reduce((acc, cur) => acc + cur, 0);
  console.log(totalExample);
};

const rowsExamplePt2 = await readData("2/input/example.txt");
solvePt2(rowsExamplePt2);

const rowsInputPt2 = await readData("2/input/input.txt");
solvePt2(rowsInputPt2);
