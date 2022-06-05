export const answer = (str: string): number | never => {
  if (!/^What is.*\?$/g.test(str)) throw 'Unknown operation'
  const parsed = str
    .replace(/(^What is )|(\?$)/g, '')
    .replace(/multiplied by/g, '%mul')
    .replace(/divided by/g, '%div')
    .replace(
      /raised to the (-?(?!1[123]\D)(\d*(1st|2nd|3rd|[^123]th))|1[123]th) power/g,
      (_, pow: string) => '%rai ' + pow.slice(0, -2)
    )
  const parsedArray = parsed.split(' ')
  let value = parseInt(parsedArray[0])
  if (Number.isNaN(value)) throw 'Syntax error'
  for (let i = 1; i < parsedArray.length; i += 2) {
    const nextVal = parseInt(parsedArray[i + 1])
    switch (parsedArray[i]) {
      case 'plus':
        value += nextVal
        break
      case 'minus':
        value -= nextVal
        break
      case '%mul':
        value *= nextVal
        break
      case '%div':
        value /= nextVal
        break
      case '%rai':
        value **= nextVal
        break
      default:
        throw Number.isNaN(parseInt(parsedArray[i]))
          ? 'Unknown operation'
          : 'Syntax error'
    }
    if (Number.isNaN(nextVal)) throw 'Syntax error'
  }
  return value
}
