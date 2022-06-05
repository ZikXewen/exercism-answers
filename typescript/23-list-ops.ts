class Node<T> {
  public val: T
  public l: Node<T> | null = null
  public r: Node<T> | null = null
  constructor(val: T) {
    this.val = val
  }
}

export class List<T> {
  private size = 0
  private head: Node<T> | null = null
  private tail: Node<T> | null = null

  private push(element: T) {
    if (this.tail) {
      this.tail.r = new Node(element)
      this.tail.r.l = this.tail
      this.tail = this.tail.r
    } else this.head = this.tail = new Node(element)
    this.size++
  }

  public forEach(func: (item: T) => any) {
    for (let node = this.head; node !== null; node = node.r) func(node.val)
  }

  public static create<T>(...val: T[]): List<T> {
    const list = new List<T>()
    for (const i of val) list.push(i)
    return list
  }

  public append(list: List<T>): List<T> {
    if (this.head === null) return list
    if (list.head === null) return this
    if (this.tail) this.tail.r = list.head
    if (list.head) list.head.l = this.tail
    this.tail = list.tail
    return this
  }

  public concat(lists: List<List<T>>): List<T> {
    lists.forEach((list) => this.append(list))
    return this
  }

  public filter(predicate: (item: T) => boolean): List<T> {
    const list = new List<T>()
    this.forEach((node) => {
      if (predicate(node)) list.push(node)
    })
    return list
  }

  public map<B>(func: (item: T) => B): List<B> {
    const list = new List<B>()
    this.forEach((node) => list.push(func(node)))
    return list
  }

  public foldl<B>(func: (acc: B, item: T) => B, init: B): B {
    this.forEach((node) => {
      init = func(init, node)
    })
    return init
  }

  public foldr<B>(func: (acc: B, item: T) => B, init: B): B {
    for (let node = this.tail; node !== null; node = node.l)
      init = func(init, node.val)
    return init
  }

  public reverse(): List<T> {
    const list = new List<T>()
    for (let node = this.tail; node !== null; node = node.l) list.push(node.val)
    return list
  }

  public length(): number {
    return this.size
  }
}
