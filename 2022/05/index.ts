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
let data: string[][] = [];

getInputs(true).forEach((row) => {
  if (row === "") foundBreak = true;

  // parse stacks
  if (!foundBreak) {
    let rowData = [...row].filter((_, index) => index % 4 === 1);
    console.log(rowData);
    data.push(rowData);
    return;
  }

  // TODO: parse commands
});

// TODO: sort stacks
// TODO: sort commands

console.log(stacks);


// console.log("part b");

// getInputs(true).forEach((row) => {
//   console.log(row);
// });
