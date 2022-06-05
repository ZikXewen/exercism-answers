type Roster = { [key: number]: string[] }

export class GradeSchool {
  private classes: Roster = {}
  public roster(): Roster {
    return JSON.parse(JSON.stringify(this.classes))
  }
  public add(str: string, grade: number) {
    Object.values(this.classes).map((ar) => {
      const id = ar.indexOf(str)
      return id === -1 ? ar : ar.splice(id, 1)
    })
    if (!this.classes[grade]) this.classes[grade] = [str]
    else {
      let fd = 0
      while (fd < this.classes[grade].length && this.classes[grade][fd] < str)
        fd++
      this.classes[grade].splice(fd, 0, str)
    }
  }
  public grade(grade: number): string[] {
    return this.roster()[grade] || []
  }
}
