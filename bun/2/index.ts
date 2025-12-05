import { readData } from "../utils";

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
  const duplicates = findDuplicates(ranges);
  console.log(duplicates);

  const totalExample = duplicates.reduce((acc, cur) => acc + cur, 0);
  console.log(totalExample);
};

const rowsExamplePt1 = await readData("2/input/example.txt");
solvePt1(rowsExamplePt1);

const rowsInputPt1 = await readData("2/input/input.txt");
solvePt1(rowsInputPt1);
