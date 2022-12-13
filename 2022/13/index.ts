import {readFileSync} from "fs";

const getInputs = (test?: boolean): string[] => {
    let data = readFileSync(`./input${test ? ".test" : ""}.txt`, {encoding: "utf-8"})
    if (!data) throw new Error("no data found");
    return data.toString().split("\r\n");
}

console.log("02");

const aMoves = ["A", "B", "C"] as const;
const bMoves = ["X", "Y", "Z"] as const;

const LOSS = 0;
const DRAW = 3;
const WIN = 6;

const getScore = (a: "A" | "B" | "C", b: "X" | "Y" | "Z"): number => {
    let aIndex = aMoves.indexOf(a);
    let bIndex = bMoves.indexOf(b);

    let score = bIndex + 1;

    if (aIndex === bIndex) return score + DRAW;

    if (a === "A") return score + (b === "Y" ? WIN : LOSS);
    if (a === "B") return score + (b === "Z" ? WIN : LOSS);
    return score + (b === "X" ? WIN : LOSS);
}

let score = 0;

type Game = `${"A" | "B" | "C"} ${"X" | "Y" | "Z"}`;

getInputs().forEach((game) => {
    let [a, b] = (game as Game).split(" ") as ["A" | "B" | "C", "X" | "Y" | "Z"];
    score += getScore(a, b);
});

console.log("score: " + score);
