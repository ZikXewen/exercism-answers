export class Series {
  private readonly arr: number[]

  constructor(series: string) {
    if (series === '') throw 'series cannot be empty'
    this.arr = series.split('').map((x) => parseInt(x))
  }

  public slices(len: number): number[][] | never {
    if (len == 0) throw 'slice length cannot be zero'
    if (len < 0) throw 'slice length cannot be negative'
    if (len > this.arr.length)
      throw 'slice length cannot be greater than series length'
    return [...Array(this.arr.length - len + 1)].map((_, i) =>
      this.arr.slice(i, i + len)
    )
  }
}
