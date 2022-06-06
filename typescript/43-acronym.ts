export const parse = (str: string): string =>
  (str.match(/[^A-Z][A-Z]|[ \-]\w|^\w/g) || [])
    .map((x) => x[x.length - 1].toUpperCase())
    .join('')
