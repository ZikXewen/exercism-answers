export class SimpleCipher {
  public readonly key: string
  private keyArray: number[]
  private keyLength: number
  constructor(key?: string) {
    if (key) {
      this.keyArray = key.split('').map((x) => x.charCodeAt(0) - 97)
      this.key = key
    } else {
      this.keyArray = [...Array(100)].map((_) => Math.floor(Math.random() * 26))
      this.key = this.keyArray.map((x) => String.fromCharCode(x + 97)).join('')
    }
    this.keyLength = this.key.length
  }
  public encode(str: string): string {
    return str
      .split('')
      .map((x, i) => {
        const oldChar = x.charCodeAt(0) - 97
        const newChar = (oldChar + this.keyArray[i % this.keyLength]) % 26
        return String.fromCharCode(newChar + 97)
      })
      .join('')
  }

  public decode(str: string): string {
    return str
      .split('')
      .map((x, i) => {
        const oldChar = x.charCodeAt(0) - 97
        const newChar =
          (((oldChar - this.keyArray[i % this.keyLength]) % 26) + 26) % 26
        return String.fromCharCode(newChar + 97)
      })
      .join('')
  }
}
