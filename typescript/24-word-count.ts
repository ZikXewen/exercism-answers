export const count = (str: string): Map<string, number> =>
  (
    str
      .toLowerCase()
      // .match(/\d+|(\w+('\w+)?)/g)
      .match(/\S+/g) || []
  ).reduce(
    (ar, va) => (ar.set(va, (ar.get(va) || 0) + 1), ar),
    new Map<string, number>()
  )
