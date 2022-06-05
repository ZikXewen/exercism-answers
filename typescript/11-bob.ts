export function hey(message: string): string {
  message = message.trim()
  if (message.length === 0) return 'Fine. Be that way!'
  const question = message.endsWith('?')
  const yell = message === message.toUpperCase() && /[A-Z]/.test(message)
  if (question) {
    if (yell) return "Calm down, I know what I'm doing!"
    else return 'Sure.'
  } else {
    if (yell) return 'Whoa, chill out!'
    else return 'Whatever.'
  }
}
