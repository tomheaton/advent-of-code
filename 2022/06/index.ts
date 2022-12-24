import {readFileSync} from "node:fs";

const getInputs = (test?: boolean): string[] => {
  let data = readFileSync(`./input${test ? ".test" : ""}.txt`, {encoding: "utf-8"});
  if (!data) throw new Error("no data found");
  return data.toString().split("\n");
}

console.log("06");

console.log("part a");

let count = 0;
let marker = "";

getInputs(true).forEach((row) => {
  [...row].forEach((char, index) => {
    // console.log(char);

    // if (marker.length === 4) return;

    // get a list of all the characters before the current character
    let prevChars = row.slice(index - 1, index);
    console.log(prevChars);

    return;


    if (!prevChars.includes(char)) {
      count++;
      marker += char;
      console.log("marker:", marker);
    } else {
      marker = "";
    }
  });
});

console.log("marker:", marker);
console.log("count:", count);

// console.log("part b");

// getInputs(true).forEach((row) => {
//   console.log(row);
// });
