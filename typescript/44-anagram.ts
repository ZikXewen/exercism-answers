export class Anagram {
  private key: string
  private cleanKey: string
  constructor(input: string) {
    this.key = input.toLowerCase()
    this.cleanKey = this.key.split('').sort().join('')
  }

  public matches(...potentials: string[]): string[] {
    return potentials.filter(
      (word) =>
        word.toLowerCase() !== this.key &&
        word.toLowerCase().split('').sort().join('') === this.cleanKey
    )
  }
}
