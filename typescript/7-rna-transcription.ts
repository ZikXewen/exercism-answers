const mapping = {
  G: 'C',
  C: 'G',
  T: 'A',
  A: 'U',
}
export function toRna(dna: string): string {
  return dna
    .split('')
    .map((c) => {
      if (c in mapping) return mapping[c as keyof typeof mapping]
      throw new Error('Invalid input DNA.')
    })
    .join('')
}
