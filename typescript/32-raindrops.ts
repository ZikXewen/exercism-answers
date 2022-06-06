export const convert = (no: number): string =>
  (no % 3 ? '' : 'Pling') + (no % 5 ? '' : 'Plang') + (no % 7 ? '' : 'Plong') ||
  no.toString()
