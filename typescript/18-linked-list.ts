class Node<T> {
  public val: T
  public l: Node<T> | null = null
  public r: Node<T> | null = null
  constructor(val: T) {
    this.val = val
  }
}

export class LinkedList<T> {
  private size = 0
  private head: Node<T> | null = null
  private tail: Node<T> | null = null
  public push(element: T) {
    if (this.tail) {
      this.tail.r = new Node(element)
      this.tail.r.l = this.tail
      this.tail = this.tail.r
    } else this.head = this.tail = new Node(element)
    this.size++
  }

  public pop(): T {
    if (!this.tail) throw new Error('Empty')
    const ret = this.tail
    this.tail = this.tail.l
    if (this.tail) this.tail.r = null
    this.size--
    if (!this.size) this.head = null
    return ret.val
  }

  public shift(): T {
    if (!this.head) throw new Error('Empty')
    const ret = this.head
    this.head = this.head.r
    if (this.head) this.head.l = null
    this.size--
    if (!this.size) this.tail = null
    return ret.val
  }

  public unshift(element: T) {
    if (this.head) {
      this.head.l = new Node(element)
      this.head.l.r = this.head
      this.head = this.head.l
    } else this.head = this.tail = new Node(element)
    this.size++
  }

  public delete(element: T) {
    for (let cr = this.head; cr; cr = cr.r) {
      if (cr.val === element) {
        if (cr.l) cr.l.r = cr.r
        if (cr.r) cr.r.l = cr.l
        this.size--
      }
    }
  }

  public count(): number {
    return this.size
  }
}
