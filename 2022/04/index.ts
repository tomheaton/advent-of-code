import {readFileSync} from "node:fs";

const getInputs = (test?: boolean): string[] => {
  let data = readFileSync(`./input${test ? ".test" : ""}.txt`, {encoding: "utf-8"});
  if (!data) throw new Error("no data found");
  return data.toString().split("\n");
}

console.log("04");

console.log("part a");

let count = 0;

getInputs().forEach((row) => {
  let aPair = row.split(",")[0];
  let bPair = row.split(",")[1];

  let a = aPair.split("-").map((n) => parseInt(n));
  let b = bPair.split("-").map((n) => parseInt(n));

  let aList = Array.from({length: a[1] - a[0] + 1}, (_, i) => a[0] + i);
  let bList = Array.from({length: b[1] - b[0] + 1}, (_, i) => b[0] + i);

  let aIsSubset = aList.every((n) => bList.includes(n));
  let bIsSubset = bList.every((n) => aList.includes(n));

  if (aIsSubset || bIsSubset) count++;
});

console.log("count: " + count);

console.log("part b");

count = 0;

getInputs().forEach((row) => {
  let aPair = row.split(",")[0];
  let bPair = row.split(",")[1];

  let a = aPair.split("-").map((n) => parseInt(n));
  let b = bPair.split("-").map((n) => parseInt(n));

  let aList = Array.from({length: a[1] - a[0] + 1}, (_, i) => a[0] + i);
  let bList = Array.from({length: b[1] - b[0] + 1}, (_, i) => b[0] + i);

  let aIsSubset = aList.some((n) => bList.includes(n));
  let bIsSubset = bList.some((n) => aList.includes(n));

  if (aIsSubset || bIsSubset) count++;
});

console.log("count: " + count);
