export function steps(cur: number): number | never {
  if (cur <= 0) throw 'Only positive numbers are allowed'
  if (cur === 1) return 0
  return 1 + steps(cur % 2 ? 3 * cur + 1 : cur / 2)
}
