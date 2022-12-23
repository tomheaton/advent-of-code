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

getInputs(true).slice(0, 1).forEach((row) => {
  console.log(row);

  let spaceCount = 0;

  if (!foundBreak) {
    for (const c of row) {

      if (c === "[") {
        let currentStack: string[] = [];
        stacks.push(currentStack);
        continue;
      }

      if (c === " ") {
        spaceCount++;
        continue;
      }
    }
  }

});

console.log(stacks);


// console.log("part b");

// getInputs(true).forEach((row) => {
//   console.log(row);
// });
