import {readFileSync} from "node:fs";

const getInputs = (test?: boolean): string[] => {
  let data = readFileSync(`./input${test ? ".test" : ""}.txt`, {encoding: "utf-8"});
  if (!data) throw new Error("no data found");
  return data.toString().split("\n");
}

console.log("07");

console.log("part a");

getInputs(true).forEach((row) => {
  let tokens = row.split(" ");

  if (tokens[0] === "$") {
    console.log("found $", tokens);
  }

  console.log(tokens);
});

// console.log("part b");

// getInputs(true).forEach((row) => {
//   console.log(row);
// });
