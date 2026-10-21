/**
 * Scrolls the text box so a mark of its backdrop is in view, a third from
 * the top or left when it was not. The backdrop lays the text out as the box
 * does, so the mark's offset is where its text sits in the box.
 */
export function scrollToMark(box: HTMLTextAreaElement, mark: HTMLElement) {
  const { offsetTop: top, offsetLeft: left, offsetWidth: width, offsetHeight: height } = mark
  if (top < box.scrollTop || top + height > box.scrollTop + box.clientHeight) {
    box.scrollTop = top - box.clientHeight / 3
  }
  if (left < box.scrollLeft || left + width > box.scrollLeft + box.clientWidth) {
    box.scrollLeft = left - box.clientWidth / 3
  }
}
