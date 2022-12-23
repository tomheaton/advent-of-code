import {readFileSync} from "fs";

const getInputs = (test?: boolean): string[] => {
  let data = readFileSync(`./input${test ? ".test" : ""}.txt`, {encoding: "utf-8"});
  if (!data) throw new Error("no data found");
  return data.toString().split("\n");
}

console.log("05");

console.log("part a");

let foundBreak = false;

let stacks: string[][] = [];
let rawData: string[][] = [];

getInputs(true).forEach((row) => {
  if (row === "") foundBreak = true;

  // parse stacks
  if (!foundBreak) {
    let rowData = [...row].filter((_, index) => index % 4 === 1);
    rawData.push(rowData);
    return;
  }

  // TODO: parse commands
});

let indexes = rawData.pop();

stacks = rawData[0]
  .map((_, colIndex) => rawData.map(row => row[colIndex]))
  .map((row) => row.filter((v) => v !== " ").reverse());

// TODO: sort commands

console.log(stacks);

// console.log("part b");

// getInputs(true).forEach((row) => {
//   console.log(row);
// });
