async function getFile (): Promise<string> {
  const path = './src/day14.test'
  return await Bun.file(path).text()
}
function parseLine (line: string): void {
  const x = line.split(' -> ').map(pair => pair.split(','))
  console.log(x)
}

async function main (): Promise<any> {
  const file = await getFile()
  const lines = file.trim().split('\n')
  const walls = lines.map(parseLine)
  console.log(res)
}

console.log(await main())
