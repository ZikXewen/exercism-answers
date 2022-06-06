export function clean(str: string): string | never {
  if (/[a-zA-Z]/.test(str)) throw 'Letters not permitted'
  if (/[^0-9\-().+ ]/.test(str)) throw 'Punctuations not permitted'
  const arr: string[] = str.match(/\d/g) || []
  if (arr.length < 10) throw 'Incorrect number of digits'
  if (arr.length > 11) throw 'More than 11 digits'
  if (arr.length === 11) {
    if (arr[0] === '1') arr.shift()
    else throw '11 digits must start with 1'
  }
  if (arr[0] == '0') throw 'Area code cannot start with zero'
  if (arr[0] == '1') throw 'Area code cannot start with one'
  if (arr[3] == '0') throw 'Exchange code cannot start with zero'
  if (arr[3] == '1') throw 'Exchange code cannot start with one'
  return arr.join('')
}
