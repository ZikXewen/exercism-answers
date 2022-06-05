const colorMapping = {
  black: 0,
  brown: 1,
  red: 2,
  orange: 3,
  yellow: 4,
  green: 5,
  blue: 6,
  violet: 7,
  grey: 8,
  white: 9,
}

type Color = keyof typeof colorMapping

export function decodedResistorValue([
  color1,
  color2,
  color3,
]: Color[]): string {
  const res =
    (colorMapping[color1] * 10 + colorMapping[color2]) *
    10 ** colorMapping[color3]
  return res >= 1000 ? `${res / 1000} kiloohms` : `${res} ohms`
}
