import { readData } from "../utils";

const calculateRow = (row: string, currentNumber: number) => {
  // Work with a copy to not mutate
  let newNumber: number;

  const letter = row.charAt(0);
  const number = Number(row.slice(1));
  const remainder = number % 100;

  // If the we flipped over from 99 to 0 (R) or from 0 to 99 (L)
  let tippingPoint: "R" | "L" | undefined = undefined;

  // handle full rotations, we don't care if it is right or left
  const numberOfFullRotations = Math.floor(number / 100);

  if (letter === "L") {
    newNumber = currentNumber - remainder;
  } else {
    newNumber = currentNumber + remainder;
  }

  if (newNumber > 99) {
    newNumber -= 100;
    tippingPoint = "R";
  }

  if (newNumber < 0) {
    newNumber += 100;
    tippingPoint = "L";
  }

  return {
    letter,
    number,
    nextNumber: newNumber,
    numberOfFullRotations,
    tippingPoint,
  };
};

const part1 = (rows: string[]) => {
  let currentNumber = 50;
  let numberOfZeros = 0;

  rows.map((row) => {
    const { nextNumber } = calculateRow(row, currentNumber);
    if (nextNumber === 0) {
      numberOfZeros++;
    }

    currentNumber = nextNumber;
  });

  return numberOfZeros;
};

const part2 = (rows: string[]) => {
  let currentNumber = 50;
  let numberOfTimesItPointsToZero = 0;

  rows.forEach((row) => {
    const startedAtZero = currentNumber === 0;
    const { nextNumber, numberOfFullRotations, tippingPoint } = calculateRow(
      row,
      currentNumber
    );

    numberOfTimesItPointsToZero += numberOfFullRotations;

    if (nextNumber === 0) {
      numberOfTimesItPointsToZero++;
    } else if (tippingPoint && !startedAtZero) {
      numberOfTimesItPointsToZero++;
    }

    currentNumber = nextNumber;
  });

  return numberOfTimesItPointsToZero;
};

const exampleData = await readData("example.txt");
const inputData = await readData("input.txt");

const resultExamplePt1 = part1(exampleData);
console.log(resultExamplePt1);

const resultInputPt1 = part1(inputData);
console.log(resultInputPt1);

const resultExamplePt2 = part2(exampleData);
console.log(resultExamplePt2);

const resultInputPt2 = part2(inputData);
console.log(resultInputPt2);
