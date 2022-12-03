import {readFileSync} from "fs";

const getInputs = (): string[] => {
    let data = readFileSync("./input.txt", {encoding: "utf-8"})
    if (!data) throw new Error("no data found");
    return data.toString().split("\r\n");
}


console.log("01");

console.log(getInputs());

let answer = 0;

getInputs().forEach((calories, index) => {
    if (!calories) {
        answer = 0;
        return;
    }
    answer += Number(calories);
});

console.log("answer: " + answer);
