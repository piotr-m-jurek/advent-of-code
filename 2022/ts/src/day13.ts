type Val = number | (Val[]);

function main() {
  const pairs = Deno.readTextFileSync("./src/day13.prod").trim().split("\n\n");

  const sum = pairs.reduce((sum, pair, index) => {
    const [left, right] = pair.split("\n").map((packet) =>
      JSON.parse(packet)
    ) as Val[];

    const isInOrder = getOrder(left, right);

    return sum + (isInOrder !== 1 ? index + 1 : 0);
  }, 0);

  console.log("Part 1: ", sum);

  // ================================================================================

  const vals = pairs.map((line) => line.split("\n"))
    .flat()
    .map((line) => JSON.parse(line) as Val);

  const divider1 = [[2]];
  const divider2 = [[6]];

  vals.push(divider1); // Find by reference
  vals.push(divider2); // Find by reference

  vals.sort(getOrder);

  let mult = 1;
  for (let i = 0; i < vals.length; i++) {
    if (
      vals.indexOf(divider1) === i ||
      vals.indexOf(divider2) === i
    ) {
      mult *= i + 1;
    }
  }

  console.log("part 2: ", mult);
}

main();

type Order = -1 | 1 | 0;

function getOrder(a: Val, b: Val): Order {
  if (typeof a === "number" && typeof b === "number") {
    return Math.sign(a - b) as Order;
  }

  const left = arraify(a);
  const right = arraify(b);

  for (let idx = 0; idx < left.length && idx < right.length; ++idx) {
    const result = getOrder(left[idx], right[idx]);
    if (result !== 0) {
      return result;
    }
  }

  return Math.sign(left.length - right.length) as Order;
}

function arraify(arr: Val) {
  return Array.isArray(arr) ? arr : [arr];
}
