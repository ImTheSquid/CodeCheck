// place files you want to import through the `$lib` alias in this folder.
export interface MarkSpan {
  start: number,
  end: number,
}
export interface Mark {
  a: MarkSpan,
  b: MarkSpan,
  color: string,
}
