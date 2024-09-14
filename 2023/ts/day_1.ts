
import * as utils from './utils'

const lines = await utils.readLines('./day_1.prod')

const answer = lines
    .map(line => {
        const parsedNumbers = line.split('').filter(Number)
        const first = parsedNumbers.at(0);
        const last =  parsedNumbers[parsedNumbers.length - 1]

        return Number(first + last)
    })
    .map(utils.log())
    .reduce((acc, el) => acc + el, 0);

console.log('test answer', answer);
