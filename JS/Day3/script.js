const { count } = require("console");
const fs = require("fs");

fs.readFile(
  "JS/Day3/adventofcode.com_2022_day_3_input.txt",
  "utf8",
  (err, data) => {
    if (err) {
      console.error(err);
      return;
    }
    console.log(partTwo(data));
  }
);

const example = `vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw`;

const charToPrio = (char) => {
  let output = char.charCodeAt(0);
  if (output >= 97) output -= 96;
  else output -= 38;
  return output;
};

const splitStringByLine = (input) => {
  const output = input.split(/\r?\n/);
  output.pop();
  return output;
};

const partString = (input) => {
  if (input.length == 0) return 0;

  for (let i = 0; i < input.length / 2; i++) {
    for (let j = input.length / 2; j < input.length; j++) {
      if (input[i] == input[j]) {
        return charToPrio(input[i]);
      }
    }
  }
};

const partOne = (input) => {
  lines = splitStringByLine(input);
  let total = 0;
  for (let i = 0; i < lines.length; i++) {
    total += partString(lines[i]);
  }
  return total;
};

const test = () => {
  const kake = [];
  kake[15] += 1;
  console.log(typeof kake[152]);
};

const partTwo = (input) => {
  const chars = [];
  let total = 0;

  const lines = splitStringByLine(input);

  for (let i = 0; i < lines.length; i++) {
    lines[i] = new Set(lines[i].split(""));
  }

  for (let i = 0; i < lines.length; i += 3) {
    const charCount = [];

    for (const e of lines[i]) {
      charCount[e.charCodeAt(0)] = 1;
    }

    for (const e of lines[i + 1]) {
      charCount[e.charCodeAt(0)] += 1;
    }

    for (const e of lines[i + 2]) {
      charCount[e.charCodeAt(0)] += 1;
    }

    for (let j = 0; j < charCount.length; j++) {
      if (charCount[j] == 3) {
        chars[chars.length] = String.fromCharCode(j);
      }
    }
  }

  console.log(chars);
  for (let i = 0; i < chars.length; i++) {
    console.log(chars[i]);
    total += charToPrio(chars[i]);
    console.log();
  }

  console.log(total);
};
