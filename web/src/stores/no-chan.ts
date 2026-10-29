import { createNooChan } from 'no-chan'

/**
 * How features tell no-chan what happened: a request sent from a button,
 * or something the user did that failed. The app puts her on the page;
 * while she is not there, what she is told is dropped.
 */
export const nooChan = createNooChan()
