/* tslint:disable */
/* eslint-disable */
/**
 * A demo function for getting WASM working for the first time
 */
export function init(): void;
/**
 * A demo function for getting WASM working for the first time
 */
export function hello_world(): Promise<void>;
/**
 * Converts a markdown string to HTML. Available in WASM only.
 */
export function md_to_html(md: string): Promise<ValidatedMarkdown>;
/**
 * Combines all the links in the logical order:
 * - question links
 * - correct answer links
 * - incorrect answer links
 *
 * All links are sorted alphabetically within their logical group
 */
export function sort_links(question_links: (string)[], correct_answer_links: (string)[], incorrect_answer_links: (string)[]): (string)[];
/**
 * Contains the result of the streaming parser that returns HTML
 * and what was filtered out in a single pass.
 */
export class ValidatedMarkdown {
  private constructor();
  free(): void;
  /**
   * The HTML representation of the markdown
   * with disallowed elements removed.
   */
  html: string;
  /**
   * The list of disallowed markdown elements that were ignored during the HTML conversion.
   */
  ignored: (string)[];
  /**
   * The list of link URLs found in the markdown.
   * The are what the parser considers a link, which may be absolute or relative.
   * The URLs are not validated and appear in the order they were encountered.
   */
  links: (string)[];
  /**
   * The list of image URLs found in the markdown.
   * The URLs are not validated and appear in the order they were encountered.
   */
  images: (string)[];
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly init: () => void;
  readonly hello_world: () => any;
  readonly md_to_html: (a: number, b: number) => any;
  readonly sort_links: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
  readonly __wbg_validatedmarkdown_free: (a: number, b: number) => void;
  readonly __wbg_get_validatedmarkdown_html: (a: number) => [number, number];
  readonly __wbg_set_validatedmarkdown_html: (a: number, b: number, c: number) => void;
  readonly __wbg_get_validatedmarkdown_ignored: (a: number) => [number, number];
  readonly __wbg_set_validatedmarkdown_ignored: (a: number, b: number, c: number) => void;
  readonly __wbg_get_validatedmarkdown_links: (a: number) => [number, number];
  readonly __wbg_set_validatedmarkdown_links: (a: number, b: number, c: number) => void;
  readonly __wbg_get_validatedmarkdown_images: (a: number) => [number, number];
  readonly __wbg_set_validatedmarkdown_images: (a: number, b: number, c: number) => void;
  readonly ring_core_0_17_8_bn_mul_mont: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_6: WebAssembly.Table;
  readonly __externref_drop_slice: (a: number, b: number) => void;
  readonly closure14_externref_shim: (a: number, b: number, c: any) => void;
  readonly closure88_externref_shim: (a: number, b: number, c: any, d: any) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
