export function find(arr: number[], x: number): number | never {
  let l = 0,
    r = arr.length - 1
  while (l < r) {
    let m = Math.floor((l + r) / 2)
    if (x <= arr[m]) r = m
    else l = m + 1
  }
  if (arr[l] !== x) throw new Error('Value not in array')
  return l
}
