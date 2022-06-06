export class TwoBucket {
  private _moves: number
  public readonly goalBucket: 'one' | 'two'
  public readonly otherBucket: number

  constructor(b1cap: number, b2cap: number, goal: number, fm: string) {
    const passed: boolean[][] = [...Array(b1cap + 1)].map((_) =>
      Array(b2cap + 1).fill(false)
    )
    passed[0][0] = passed[b1cap][0] = passed[0][b2cap] = true
    const queue =
      fm === 'one'
        ? [{ b1: b1cap, b2: 0, moves: 1 }]
        : [{ b1: 0, b2: b2cap, moves: 1 }]
    for (let point = 0; point < queue.length; point++) {
      let { b1, b2, moves } = queue[point]
      if (b1 === goal) {
        this._moves = moves
        this.goalBucket = 'one'
        this.otherBucket = b2
        return
      }
      if (b2 === goal) {
        this._moves = moves
        this.goalBucket = 'two'
        this.otherBucket = b1
        return
      }
      const pass = (a: number, b: number) => {
        if (!passed[a][b]) {
          passed[a][b] = true
          queue.push({ b1: a, b2: b, moves: moves + 1 })
        }
      }
      pass(b1, 0)
      pass(0, b2)
      pass(b1cap, b2)
      pass(b1, b2cap)
      const to2 = Math.min(b2cap - b2, b1)
      pass(b1 - to2, b2 + to2)
      const to1 = Math.min(b1cap - b1, b2)
      pass(b1 + to1, b2 - to1)
    }
    throw 'Not possible to reach the goal'
  }

  public moves() {
    return this._moves
  }
}
