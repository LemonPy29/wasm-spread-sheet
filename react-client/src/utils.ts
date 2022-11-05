export function inferColumnName(node: HTMLElement): HTMLElement {
  const childs = node.childNodes;
  return childs.length > 1
    ? (childs.item(0) as HTMLDivElement)
    : inferColumnName(node.parentElement!);
}

export function contextMenuPosition(posX: number): "left" | "right" {
  const vw = Math.max(document.documentElement.clientWidth || 0, window.innerWidth || 0);
  return posX <= vw / 1.5 ?  "left" : "right";
}
