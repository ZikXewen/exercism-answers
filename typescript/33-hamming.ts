export function compute(a: string, b: string): number | never {
  if (a.length !== b.length) throw 'DNA strands must be of equal length.'
  return [...a].filter((x, i) => x !== b[i]).length
}
