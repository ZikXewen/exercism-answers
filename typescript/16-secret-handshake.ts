export function commands(num: number): string[] {
  const str = num.toString(2).padStart(5)
  const cmd: string[] = []
  if (str[4] === '1') cmd.push('wink')
  if (str[3] === '1') cmd.push('double blink')
  if (str[2] === '1') cmd.push('close your eyes')
  if (str[1] === '1') cmd.push('jump')
  if (str[0] === '1') cmd.reverse()
  return cmd
}
