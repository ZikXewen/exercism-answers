export function valid(dig: string): boolean {
  if (!/^(\d| )+$/.test(dig)) return false
  if (dig.trim() === '0') return false
  return (
    (dig.match(/\d/g) || []).reverse().reduce((cr, x, i) => {
      let no = parseInt(x) * (i % 2 === 0 ? 1 : 2)
      if (no >= 10) no -= 9
      return cr + no
    }, 0) %
      10 ===
    0
  )
}
