type Options = {
  minFactor?: number
  maxFactor?: number
  sum: number
}

export function triplets({ minFactor, maxFactor, sum }: Options): Triplet[] {
  const ret: Triplet[] = []
  for (let c = Math.floor(sum / 2); c >= sum / 3; c--)
    for (let a = 1; a <= sum - c - a; a++)
      if (
        a ** 2 + (sum - c - a) ** 2 === c ** 2 &&
        (!minFactor || a >= minFactor) &&
        (!maxFactor || c <= maxFactor)
      )
        ret.push(new Triplet(a, sum - c - a, c))
  return ret
}

class Triplet {
  private sides: [number, number, number]

  constructor(a: number, b: number, c: number) {
    this.sides = [a, b, c]
  }

  toArray(): [number, number, number] {
    return this.sides
  }
}
