export class ComplexNumber {
  public readonly real: number
  public readonly imag: number
  constructor(real: number, imag: number) {
    if (real === -0) real = Math.abs(real)
    if (imag === -0) imag = Math.abs(imag)
    this.real = real
    this.imag = imag
  }

  public add(other: ComplexNumber): ComplexNumber {
    return new ComplexNumber(this.real + other.real, this.imag + other.imag)
  }

  public sub(other: ComplexNumber): ComplexNumber {
    return new ComplexNumber(this.real - other.real, this.imag - other.imag)
  }

  public mul(other: ComplexNumber): ComplexNumber {
    return new ComplexNumber(
      this.real * other.real - this.imag * other.imag,
      this.imag * other.real + this.real * other.imag
    )
  }

  public div(other: ComplexNumber): ComplexNumber {
    const div = other.real * other.real + other.imag * other.imag
    return new ComplexNumber(
      (this.real * other.real + this.imag * other.imag) / div,
      (this.imag * other.real - this.real * other.imag) / div
    )
  }

  public get abs(): number {
    return Math.sqrt(this.real * this.real + this.imag * this.imag)
  }

  public get conj(): ComplexNumber {
    return new ComplexNumber(this.real, -this.imag)
  }

  public get exp(): ComplexNumber {
    const exp = Math.exp(this.real)
    return new ComplexNumber(
      exp * Math.cos(this.imag),
      exp * Math.sin(this.imag)
    )
  }
}
