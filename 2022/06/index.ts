import {readFileSync} from "node:fs";

const getInputs = (test?: boolean): string[] => {
  let data = readFileSync(`./input${test ? ".test" : ""}.txt`, {encoding: "utf-8"});
  if (!data) throw new Error("no data found");
  return data.toString().split("\n");
}

console.log("06");

console.log("part a");

let marker: string[] = [];

getInputs().forEach((row) => {
  [...row].some((char, index) => {
    marker.push(char);

    if (marker.length > 4) {
      marker.shift();
    }

    if (marker.length === 4 && (new Set(marker)).size === marker.length) {
      console.log("location:", index + 1);
      return true;
    }
  });
});

// console.log("part b");

// getInputs(true).forEach((row) => {
//   console.log(row);
// });
