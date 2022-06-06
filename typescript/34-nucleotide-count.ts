type Nucleotide = 'A' | 'C' | 'G' | 'T'
const isNucleotide = (str: string): str is Nucleotide => 'ACGT'.includes(str)
export const nucleotideCounts = (
  str: string
): Record<Nucleotide, number> | never =>
  [...str].reduce(
    (cr, x) => {
      if (!isNucleotide(x)) throw 'Invalid nucleotide in strand'
      cr[x]++
      return cr
    },
    {
      A: 0,
      C: 0,
      G: 0,
      T: 0,
    }
  )
