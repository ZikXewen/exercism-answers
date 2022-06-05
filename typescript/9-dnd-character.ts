export class DnDCharacter {
  private strength = DnDCharacter.generateAbilityScore()
  private dexterity = DnDCharacter.generateAbilityScore()
  private constitution = DnDCharacter.generateAbilityScore()
  private intelligence = DnDCharacter.generateAbilityScore()
  private wisdom = DnDCharacter.generateAbilityScore()
  private charisma = DnDCharacter.generateAbilityScore()
  public hitpoints: number

  constructor() {
    this.hitpoints = 10 + DnDCharacter.getModifierFor(this.constitution)
  }

  public static generateAbilityScore(): number {
    const arr = [...Array(4)].map((_) => Math.ceil(Math.random() * 6))
    return arr.reduce((cr, x) => cr + x) - Math.min(...arr)
  }

  public static getModifierFor(abilityValue: number): number {
    return Math.floor((abilityValue - 10) / 2)
  }
}
