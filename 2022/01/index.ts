import {readFileSync} from "node:fs";

const getInputs = (test?: boolean): string[] => {
    let data = readFileSync(`./input${test ? ".test" : ""}.txt`, {encoding: "utf-8"});
    if (!data) throw new Error("no data found");
    return data.toString().split("\n");
}

console.log("01");

let accumulator = 0;
let elves: number[] = []

getInputs().forEach((calories) => {
    if (!calories) {
        elves.push(accumulator);
        accumulator = 0;
        return;
    }
    accumulator += Number(calories);
});
elves.push(accumulator)

elves = elves.sort((a, b) => b - a);
let topOne = elves.slice(0, 1).reduce((acc, cur) => acc + cur, 0)
let topThree = elves.slice(0, 3).reduce((acc, cur) => acc + cur, 0)

console.log("topOne: " + topOne);
console.log("topThree: " + topThree);
