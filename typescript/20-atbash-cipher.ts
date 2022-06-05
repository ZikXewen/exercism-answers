export const encode = (str: string) =>
  (str.toLowerCase().match(/[a-z0-9]/g) || [])
    .map(
      (s, i) =>
        (i % 5 || !i ? '' : ' ') +
        (/[0-9]/.test(s) ? s : String.fromCharCode(219 - s.charCodeAt(0)))
    )
    .join('')

export const decode = (str: string) =>
  str
    .split('')
    .map((s) => {
      if (s === ' ') return ''
      if (/[0-9]/.test(s)) return s
      return String.fromCharCode(219 - s.charCodeAt(0))
    })
    .join('')
