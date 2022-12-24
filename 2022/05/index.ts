import { strictEqual } from "assert";
import {readFileSync} from "node:fs";

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

let commands: Command[] = [];

type Command = {
  amount: number;
  to: number;
  from: number;
}

getInputs().forEach((row) => {
  if (row === "") {
    foundBreak = true;
    return;
  }

  // parse stacks
  if (!foundBreak) {
    let rowData = [...row].filter((_, index) => index % 4 === 1);
    rawData.push(rowData);
    return;
  }

  // parse commands
  let numbers = row.match(/\d+/g);
  if (!numbers) throw new Error("no numbers found");

  commands.push({
    amount: parseInt(numbers[0]),
    from: parseInt(numbers[1]),
    to: parseInt(numbers[2]),
    });
});

rawData.pop();

stacks = rawData[0]
  .map((_, colIndex) => rawData.map(row => row[colIndex]))
  .map((row) => row.filter((v) => v !== " ").reverse());

commands.forEach((command) => {
  let fromStack = stacks[command.from - 1];
  let toStack = stacks[command.to - 1];

  let items = fromStack.splice(fromStack.length - command.amount, command.amount);
  // toStack.push(...items.reverse()); // part a
  toStack.push(...items); // part b
});

let message = stacks.map((stack) => {
  return stack.pop();
}).join("");

console.log("message:", message);

// console.log("part b");

// getInputs(true).forEach((row) => {
//   console.log(row);
// });
