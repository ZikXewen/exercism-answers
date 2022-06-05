export function isPangram(str: string) {
  const mp: { [key: string]: boolean } = {}
  str
    .toLowerCase()
    .split('')
    .forEach((c) => {
      if (c >= 'a' && c <= 'z') mp[c] = true
    })
  return Object.keys(mp).length === 26
}
