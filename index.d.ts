/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export declare function sum(a: number, b: number): number
export interface WindowPosition {
  x: number
  y: number
  width: number
  height: number
}
export interface ActiveWindow {
  title: string
  processPath: string
  appName: string
  windowId: string
  processId: number
  position: WindowPosition
}
export declare function getActiveWindow(): ActiveWindow
