export class Allergies {
  private static allergies = [
    'eggs',
    'peanuts',
    'shellfish',
    'strawberries',
    'tomatoes',
    'chocolate',
    'pollen',
    'cats',
  ]
  private id: number

  constructor(allergenIndex: number) {
    this.id = allergenIndex & 255
  }

  public list(): string[] {
    return Allergies.allergies.filter((_, i) => this.id & (1 << i))
  }

  public allergicTo(allergen: string): boolean {
    return (this.id & (1 << Allergies.allergies.indexOf(allergen))) !== 0
  }
}
