export class Robot {
  private static readonly POSSIBLE_IDS = 26 * 26 * 1000
  private static usedIds = new Array<boolean>(this.POSSIBLE_IDS)

  public static releaseNames() {
    this.usedIds = new Array<boolean>(this.POSSIBLE_IDS)
  }
  private static parseId(id: number): string {
    const d = (id % 1000).toString().padStart(3, '0')
    id = Math.floor(id / 1000)
    const c2 = String.fromCharCode(65 + (id % 26))
    id = Math.floor(id / 26)
    const c1 = String.fromCharCode(65 + id)
    return c1 + c2 + d
  }

  private id: number = 0
  public name: string = ''

  constructor() {
    this.generateId()
  }

  private generateId() {
    let runningId = Math.floor(Math.random() * Robot.POSSIBLE_IDS)
    while (Robot.usedIds[runningId])
      runningId = (runningId + 1) % Robot.POSSIBLE_IDS
    Robot.usedIds[runningId] = true
    this.id = runningId
    this.name = Robot.parseId(this.id)
  }
  public resetName() {
    // const oldId = this.id
    this.generateId()
    // Robot.usedIds[oldId] = false
  }
}
