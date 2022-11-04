/* tslint:disable */
/* eslint-disable */
/**
*/
export class ImageConverter {
  free(): void;
/**
* @param {HTMLCanvasElement} canvas
* @returns {ImageConverter}
*/
  static from_canvas(canvas: HTMLCanvasElement): ImageConverter;
/**
* @param {number} radian
*/
  rotate(radian: number): void;
/**
* @param {HTMLCanvasElement} canvas
*/
  to_canvas(canvas: HTMLCanvasElement): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_imageconverter_free: (a: number) => void;
  readonly imageconverter_from_canvas: (a: number) => number;
  readonly imageconverter_rotate: (a: number, b: number) => void;
  readonly imageconverter_to_canvas: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
