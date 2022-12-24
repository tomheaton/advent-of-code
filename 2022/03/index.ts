import {readFileSync} from "node:fs";

const getInputs = (test?: boolean): string[] => {
  let data = readFileSync(`./input${test ? ".test" : ""}.txt`, {encoding: "utf-8"});
  if (!data) throw new Error("no data found");
  return data.toString().split("\n");
}

console.log("03");

console.log("part a");

let priorities = 0;

getInputs().forEach((row) => {
  let n = row.length / 2;
  let [a, b] = [row.slice(0, n), row.slice(n)];

  let commonList = a.split("").filter((char) => b.includes(char));
  let commonItem = commonList[0];

  let commonItemNumber = commonItem.charCodeAt(0) - 96;
  if (commonItem >= "A" && commonItem <= "Z") {
    commonItemNumber += 58;
  }

  priorities += commonItemNumber;
});

console.log("priorities: " + priorities);

console.log("part b");

priorities = 0;

let group: string[] = [];

getInputs().forEach((row) => {
  if (group.length === 3) group = [];
  group.push(row);

  if (group.length === 3) {
    let commonList = group[0].split("").filter((char) => group[1].includes(char) && group[2].includes(char));
    let commonItem = commonList[0];

    let commonItemNumber = commonItem.charCodeAt(0) - 96;
    if (commonItem >= "A" && commonItem <= "Z") commonItemNumber += 58;

    priorities += commonItemNumber;
  }
});

console.log("priorities: " + priorities);
