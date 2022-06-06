export function classify(no: number) {
  if (no === 1) return 'deficient'
  if (no <= 0 || no % 1 !== 0)
    throw 'Classification is only possible for natural numbers.'
  let sum = 1
  let i = 2
  for (; i * i < no; i++) if (no % i === 0) sum += i + no / i
  if (i * i === no) no += i
  if (sum === no) return 'perfect'
  if (sum > no) return 'abundant'
  return 'deficient'
}
