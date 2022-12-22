import {readFileSync} from "fs";

const getInputs = (test?: boolean): string[] => {
  let data = readFileSync(`./input${test ? ".test" : ""}.txt`, {encoding: "utf-8"})
  if (!data) throw new Error("no data found");
  return data.toString().split("\n");
}

console.log("03");

console.log("\npart a");

let priorities = 0;

getInputs(true).forEach((row) => {
  let n = row.length / 2;
  let [a, b] = [row.slice(0, n), row.slice(n)];

  let commonList = a.split("").filter((char) => b.includes(char));
  let commonItem = commonList[0];

  // convert commonItem to a number where a-z is 1-26 and A-Z is 27-52
  let commonItemNumber = "a".charCodeAt(0) - 96;
  // if (commonItemNumber > 26) commonItemNumber -= 6;

  console.log(commonItem, commonItemNumber);
});

console.log("priorities: " + priorities);

// console.log("\npart b");

// getInputs(true).forEach((row) => {
//   console.log(row);
// });
