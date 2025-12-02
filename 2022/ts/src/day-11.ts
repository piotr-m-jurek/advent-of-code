const testInput = `
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
`;
const prodInput = `
Monkey 0:
  Starting items: 74, 64, 74, 63, 53
  Operation: new = old * 7
  Test: divisible by 5
    If true: throw to monkey 1
    If false: throw to monkey 6

Monkey 1:
  Starting items: 69, 99, 95, 62
  Operation: new = old * old
  Test: divisible by 17
    If true: throw to monkey 2
    If false: throw to monkey 5

Monkey 2:
  Starting items: 59, 81
  Operation: new = old + 8
  Test: divisible by 7
    If true: throw to monkey 4
    If false: throw to monkey 3

Monkey 3:
  Starting items: 50, 67, 63, 57, 63, 83, 97
  Operation: new = old + 4
  Test: divisible by 13
    If true: throw to monkey 0
    If false: throw to monkey 7

Monkey 4:
  Starting items: 61, 94, 85, 52, 81, 90, 94, 70
  Operation: new = old + 3
  Test: divisible by 19
    If true: throw to monkey 7
    If false: throw to monkey 3

Monkey 5:
  Starting items: 69
  Operation: new = old + 5
  Test: divisible by 3
    If true: throw to monkey 4
    If false: throw to monkey 2

Monkey 6:
  Starting items: 54, 55, 58
  Operation: new = old + 7
  Test: divisible by 11
    If true: throw to monkey 1
    If false: throw to monkey 5

Monkey 7:
  Starting items: 79, 51, 83, 88, 93, 76
  Operation: new = old * 3
  Test: divisible by 2
    If true: throw to monkey 0
    If false: throw to monkey 6
`;

function parseRawTest(lines: string[]) {
  return {
    divisibleBy: +(lines[0].match(/[0-9]+/)?.[0] ?? 0),
    correctThrow: +(lines[1].match(/[0-9]+/)?.[0] ?? 0),
    incorrectThrow: +(lines[2].match(/[0-9]+/)?.[0] ?? 0),
  };
}

function evalTest(
  stuff: ReturnType<typeof parseRawTest>,
  item: number,
): number {
  return item % stuff.divisibleBy === 0
    ? stuff.correctThrow
    : stuff.incorrectThrow;
}

function evalOp(rawOp: string, o: number) {
  const op = rawOp.replace("new =", "").trim();
  return eval(op.replace(/old/gmi, `${o}`));
}

function reactToBoredom(stressLevel: number) {
  return Math.floor(stressLevel / 3);
}

function run(input: string) {
  const monkeys = input
    .trim()
    .split("\n\n")
    .map((monkey) => {
      const [rawIndex, rawStartingItems, rawOperation, ...rawTest] = monkey
        .split(
          "\n",
        );
      return {
        id: +(rawIndex.match(/[0-9]+/)?.[0] ?? 0),
        startingItems: rawStartingItems
          .replace("Starting items: ", "")
          .split(",")
          .map((x) => +x),
        operation: rawOperation.replace("Operation: ", "").trim(),
        test: parseRawTest(
          rawTest
            .join("\n")
            .replace("Test: ", "").split("\n"),
        ),
      };
    });
  const divider = monkeys.map(({ test }) => test.divisibleBy).reduce(
    (a, b) => {
      return a * b;
    },
    1,
  );

  console.log("Before: ", monkeys);
  const log: number[] = Array.from({ length: monkeys.length }, () => 0);
  let counter = 10_000;

  while (counter > 0) {
    for (const monkey of monkeys) {
      const items = monkey.startingItems;

      while (items.length) {
        log[monkey.id] += 1;
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        let item = items.shift()!;
        item = evalOp(monkey.operation, item);
        item = item % divider;
        const nextIdx = evalTest(monkey.test, item);

        monkeys[nextIdx].startingItems.push(item);
      }
    }

    counter--;
  }
  console.log("After: ", monkeys);
  return log;
}
const resultMap = run(prodInput);
resultMap.sort((a, b) => b - a);
const [first, second] = resultMap;
console.log(resultMap, first, second, first * second);
