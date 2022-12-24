import {readFileSync} from "node:fs";

const getInputs = (test?: boolean): string[] => {
  let data = readFileSync(`./input${test ? ".test" : ""}.txt`, {encoding: "utf-8"});
  if (!data) throw new Error("no data found");
  return data.toString().split("\n");
}

console.log("06");

console.log("part a");

let count = 0;

getInputs(true).forEach((row) => {
  [...row].forEach((char, index) => {
    // console.log(char);

    let prevChars = row.slice(0, index);

    if (!prevChars.includes(char)) {
      count++;
    } else {
      count = 0;
    }

    console.log("count:", count);
  });
});

console.log("count:", count);

// console.log("part b");

// getInputs(true).forEach((row) => {
//   console.log(row);
// });
