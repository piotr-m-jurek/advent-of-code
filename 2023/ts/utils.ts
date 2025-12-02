export async function readLines (path: string): Promise<string[]> {
    const text = await Bun.file(path).text()
    return text.split('\n');
}

export function log<T = unknown>(msg: string = '') {
    let counter = 0
    return (val: T) => {
        console.log(`[${counter} ${msg}]: ${val}`)
        return val
    }
}
