export const square = (sq: number) => {
  if (sq % 1 || sq < 1 || sq > 64) throw ''
  return 2n ** BigInt(sq - 1)
}

export const total = () => 2n ** 64n - 1n
