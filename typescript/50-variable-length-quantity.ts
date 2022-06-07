export function encode(nums: number[]): number[] {
  nums.reverse()
  let arr = []
  for (let num of nums) {
    let first = true
    while (num !== 0 || first) {
      arr.push((num % 0x80) + (first ? 0 : 0x80))
      num = Math.floor(num / 0x80)
      first = false
    }
  }
  return arr.reverse()
}

export function decode(codes: number[]) {
  if (codes[codes.length - 1] >= 0x80) throw 'Incomplete sequence'
  let arr = [0]
  for (let code of codes) {
    if (code < 0x80) {
      arr[arr.length - 1] += code
      arr.push(0)
    } else {
      arr[arr.length - 1] += code - 0x80
      arr[arr.length - 1] *= 128
    }
  }
  arr.pop()
  return arr
}
