const components: Record<string, number> = {
  M: 1000,
  CM: 900,
  D: 500,
  CD: 400,
  C: 100,
  XC: 90,
  L: 50,
  XL: 40,
  X: 10,
  IX: 9,
  V: 5,
  IV: 4,
  I: 1,
}
export const toRoman = (no: number): string =>
  Object.entries(components).reduce((cr, [com, val]) => {
    while (no >= val) {
      cr += com
      no -= val
    }
    return cr
  }, '')
