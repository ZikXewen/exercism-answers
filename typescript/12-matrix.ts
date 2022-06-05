export class Matrix {
  public rows: number[][]
  public columns: number[][]
  constructor(str: string) {
    this.rows = str.split('\n').map((s) => s.split(' ').map((c) => parseInt(c)))
    this.columns = this.rows[0].map((_, i) => this.rows.map((row) => row[i]))
  }
}
