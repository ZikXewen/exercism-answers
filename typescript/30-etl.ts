type Old = { [key: number]: string[] }
type New = { [key: string]: number }
export const transform = (old: Old): New =>
  Object.entries(old).reduce<New>(
    (cur, [grp, vals]) => (
      vals.forEach((val) => (cur[val.toLowerCase()] = parseInt(grp))), cur
    ),
    {}
  )
