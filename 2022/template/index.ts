import {readFileSync} from "node:fs";

const getInputs = (test?: boolean): string[] => {
  let data = readFileSync(`./input${test ? ".test" : ""}.txt`, {encoding: "utf-8"});
  if (!data) throw new Error("no data found");
  return data.toString().split("\n");
}

console.log("00");

console.log("part a");

getInputs(true).forEach((row) => {
  console.log(row);
});

// console.log("part b");

// getInputs(true).forEach((row) => {
//   console.log(row);
// });
