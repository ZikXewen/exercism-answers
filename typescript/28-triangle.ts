export class Triangle {
  public isEquilateral: boolean = false
  public isIsosceles: boolean = false
  public isScalene: boolean = false
  constructor(...sides: number[]) {
    const tri: number[] = sides.slice(0, 3).sort()
    if (tri[0] === 0 || tri[0] + tri[1] <= tri[2]) return
    this.isEquilateral = tri[0] === tri[2]
    this.isIsosceles = tri[0] === tri[1] || tri[1] === tri[2]
    this.isScalene = !this.isIsosceles
  }
}
