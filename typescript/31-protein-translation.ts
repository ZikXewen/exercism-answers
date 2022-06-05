export function translate(str: string) {
  const arr = str.match(/.{3}/g) || []
  const stop = arr.findIndex((x) => /UAA|UAG|UGA/.test(x))
  return arr.slice(0, stop === -1 ? undefined : stop).map((x) => {
    switch (x) {
      case 'AUG':
        return 'Methionine'
      case 'UUU':
      case 'UUC':
        return 'Phenylalanine'
      case 'UUA':
      case 'UUG':
        return 'Leucine'
      case 'UCU':
      case 'UCC':
      case 'UCA':
      case 'UCG':
        return 'Serine'
      case 'UAU':
      case 'UAC':
        return 'Tyrosine'
      case 'UGU':
      case 'UGC':
        return 'Cysteine'
      case 'UGG':
        return 'Tryptophan'
    }
  })
}
