export class Squares {
  private count: number
  public readonly sumOfSquares: number
  public readonly squareOfSum: number
  public readonly difference: number
  constructor(count: number) {
    this.count = count
    this.sumOfSquares =
      (this.count * (this.count + 1) * (2 * this.count + 1)) / 6
    this.squareOfSum = ((this.count * (this.count + 1)) / 2) ** 2
    this.difference = Math.abs(this.squareOfSum - this.sumOfSquares)
  }
}
