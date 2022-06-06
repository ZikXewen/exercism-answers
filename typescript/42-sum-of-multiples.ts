export function sum(fac: number[], max: number): number {
  const arr: boolean[] = new Array(max).fill(false)
  for (let i of fac) if (i) for (let j = i; j < max; j += i) arr[j] = true
  return arr.reduce((cr, x, i) => cr + (x ? i : 0), 0)
}
