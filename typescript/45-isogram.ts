export const isIsogram = (input: string): boolean =>
  !(input.toLowerCase().match(/\w/g) || [])
    .sort()
    .some((x, i, arr) => i && x === arr[i - 1])
