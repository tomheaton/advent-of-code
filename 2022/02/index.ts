import {readFileSync} from "node:fs";

const getInputs = (test?: boolean): string[] => {
  let data = readFileSync(`./input${test ? ".test" : ""}.txt`, {encoding: "utf-8"});
  if (!data) throw new Error("no data found");
  return data.toString().split("\n");
}

console.log("02");

console.log("part a");

const aMoves = ["A", "B", "C"] as const;
const bMoves = ["X", "Y", "Z"] as const;

type Game = `${typeof aMoves[number]} ${typeof bMoves[number]}`;

const LOSS = 0;
const DRAW = 3;
const WIN = 6;

const getScore = (a: typeof aMoves[number], b: typeof bMoves[number]): number => {
  let aIndex = aMoves.indexOf(a);
  let bIndex = bMoves.indexOf(b);

  let score = bIndex + 1;

  if (aIndex === bIndex) return score + DRAW;

  if (a === "A") return score + (b === "Y" ? WIN : LOSS);
  if (a === "B") return score + (b === "Z" ? WIN : LOSS);
  return score + (b === "X" ? WIN : LOSS);
}

let score = 0;

getInputs().forEach((game) => {
  let [a, b] = (game as Game).split(" ") as [typeof aMoves[number], typeof bMoves[number]];
  score += getScore(a, b);
});

console.log("score: " + score);

console.log("part b");

score = 0;

const getScoreNew = (a: typeof aMoves[number], b: typeof bMoves[number]): number => {
  let aIndex = aMoves.indexOf(a);

  // loss
  if (b === "X") {
    let moveToLoseIndex = (aIndex + 2) % 3;
    console.log(a, "needs", aMoves[moveToLoseIndex], "to lose");
    return 1 + moveToLoseIndex + LOSS;
  }

  // draw
  if (b === "Y") {
    let moveToDrawIndex = (aIndex + 3) % 3;
    console.log(a, "needs", aMoves[moveToDrawIndex], "to draw");
    return 1 + moveToDrawIndex + DRAW;
  }

  // win
  let moveToWinIndex = (aIndex + 1) % 3;
  console.log(a, "needs", aMoves[moveToWinIndex], "to win");
  return 1 + moveToWinIndex + WIN;
}

getInputs().forEach((game) => {
  let [a, b] = (game as Game).split(" ") as [typeof aMoves[number], typeof bMoves[number]];
  score += getScoreNew(a, b);
});

console.log("score: " + score);
