export class Clock {
  private readonly time: number
  constructor(hour: number, minute?: number) {
    this.time = (((hour * 60 + (minute || 0)) % 1440) + 1440) % 1440
  }

  public toString(): string {
    const hour = Math.floor(this.time / 60)
    const minute = this.time % 60
    return (
      hour.toString().padStart(2, '0') +
      ':' +
      minute.toString().padStart(2, '0')
    )
  }

  public plus(minutes: number): Clock {
    return new Clock(0, this.time + minutes)
  }
  public minus(minutes: number): Clock {
    return new Clock(0, this.time - minutes)
  }
  public equals(other: Clock): boolean {
    return this.time === other.time
  }
}
