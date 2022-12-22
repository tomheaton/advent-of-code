import {readFileSync} from "fs";

const getInputs = (test?: boolean): string[] => {
  let data = readFileSync(`./input${test ? ".test" : ""}.txt`, {encoding: "utf-8"})
  if (!data) throw new Error("no data found");
  return data.toString().split("\r\n");
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
  console.log("a -> here");

  let aIndex = aMoves.indexOf(a);
  let bIndex = bMoves.indexOf(b);

  let score = bIndex + 1;

  if (aIndex === bIndex) return score + DRAW;

  if (a === "A") return score + (b === "Y" ? WIN : LOSS);
  if (a === "B") return score + (b === "Z" ? WIN : LOSS);
  return score + (b === "X" ? WIN : LOSS);
}

let score = 0;

getInputs(true).forEach((game) => {
  let [a, b] = (game as Game).split(" ") as [typeof aMoves[number], typeof bMoves[number]];
  console.log(getScore(a, b));

  score += getScore(a, b);
});

console.log("score: " + score);

console.log("part b");

score = 0;

const getScoreNew = (a: typeof aMoves[number], b: typeof bMoves[number]): number => {
  console.log("b -> here");

  let aIndex = aMoves.indexOf(a);

  let moveToLoseIndex = (aIndex + 1) % 3;
  let moveToDrawIndex = (aIndex + 3) % 3;
  let moveToWinIndex = (aIndex + 2) % 3;

  // lose
  if (b === "X") {
    console.log(a, "needs", aMoves[moveToLoseIndex], "to lose");
    return (1 + moveToLoseIndex + LOSS);
  }

  // draw
  if (b === "Y") {
    console.log(a, "needs", aMoves[moveToDrawIndex], "to draw");
    return (1 + moveToDrawIndex + DRAW);
  }

  // win
  console.log(a, "needs", aMoves[moveToWinIndex], "to win");
  return (1 + moveToWinIndex + WIN);
}

getInputs(true).forEach((game) => {
  console.log("game:", game);

  let [a, b] = (game as Game).split(" ") as [typeof aMoves[number], typeof bMoves[number]];
  score += getScore(a, b);
});

console.log("score: " + score);
