import {readFileSync} from "fs";

const getInputs = (): string[] => {
    let data = readFileSync("./input.txt", {encoding: "utf-8"})
    if (!data) throw new Error("no data found");
    return data.toString().split("\r\n");
}

console.log("01");

let max = 0;
let current = 0;

getInputs().forEach((calories) => {
    if (!calories) {
        if (current > max) {
            max = current;
        }
        current = 0;
        return;
    }
    current += Number(calories);
});

console.log("max: " + max);
