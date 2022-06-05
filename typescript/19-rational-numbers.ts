export class Rational {
  private static gcd(a: number, b: number): number {
    return b === 0 ? a : Rational.gcd(b, a % b)
  }
  private a: number
  private b: number

  constructor(a: number, b: number) {
    const gcd = Rational.gcd(a, b)
    this.a = a / gcd
    this.b = b / gcd
  }
  public add(x: Rational): Rational {
    return new Rational(this.a * x.b + this.b * x.a, this.b * x.b).reduce()
  }
  public sub(x: Rational): Rational {
    return new Rational(this.a * x.b - this.b * x.a, this.b * x.b).reduce()
  }
  public mul(x: Rational): Rational {
    return new Rational(this.a * x.a, this.b * x.b).reduce()
  }
  public div(x: Rational): Rational {
    return new Rational(this.a * x.b, this.b * x.a).reduce()
  }
  public abs(): Rational {
    return new Rational(Math.abs(this.a), Math.abs(this.b))
  }
  public exprational(x: number): Rational {
    return new Rational(this.a ** Math.abs(x), this.b ** Math.abs(x))
  }
  public expreal(x: number): number {
    return parseFloat((x ** (this.a / this.b)).toPrecision(15))
  }
  public reduce(): Rational {
    if (this.b < 0) {
      this.a = -this.a
      this.b = -this.b
    }
    if (this.a === -0) this.a = 0
    const gcd = Rational.gcd(Math.abs(this.a), Math.abs(this.b))
    return new Rational(this.a / gcd, this.b / gcd)
  }
}
