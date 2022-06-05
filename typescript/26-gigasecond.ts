export class Gigasecond {
  private oldMS: number
  constructor(date: Date) {
    this.oldMS = date.getTime()
  }
  public date(): Date {
    return new Date(this.oldMS + 1e12)
  }
}
