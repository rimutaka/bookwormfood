/* tslint:disable */
/* eslint-disable */
/**
* The main entry point for the UI thread to request book data.
* Multiple responses are sent back via `progress.js` to the UI thread.
* See `fn report_progress()` for more details.
* @param {string} isbn
* @param {string | undefined} [id_token]
* @param {string | undefined} [share_id]
* @returns {Promise<void>}
*/
export function get_book_data(isbn: string, id_token?: string, share_id?: string): Promise<void>;
/**
* Returns the list of previously scanned books from the local storage.
* See `fn report_progress()` for more details.
* @param {string | undefined} id_token
* @param {boolean} with_cloud_sync
* @returns {Promise<void>}
*/
export function get_scanned_books(id_token: string | undefined, with_cloud_sync: boolean): Promise<void>;
/**
* Updates the status of a book in the local storage.
* Returns `WasmResponse::LocalBook::Ok` in a message if successful.
* @param {string} isbn
* @param {ReadStatus | undefined} [status]
* @param {string | undefined} [id_token]
* @returns {Promise<void>}
*/
export function update_book_status(isbn: string, status?: ReadStatus, id_token?: string): Promise<void>;
/**
* Deletes a book from the local storage.
* Returns error or success via an async message.
* @param {string} isbn
* @param {string | undefined} [id_token]
* @returns {Promise<void>}
*/
export function delete_book(isbn: string, id_token?: string): Promise<void>;
/**
* Uploads a file to S3.
* Returns error or success via an async message.
* @param {string} isbn
* @param {FileList} files
* @param {string | undefined} [id_token]
* @returns {Promise<void>}
*/
export function upload_pic(isbn: string, files: FileList, id_token?: string): Promise<void>;
/**
* Where the reader is with the book.
* Defaults to None.
*/
export enum ReadStatus {
  ToRead = 0,
  Read = 1,
  Liked = 2,
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly get_book_data: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly get_scanned_books: (a: number, b: number, c: number) => number;
  readonly update_book_status: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly delete_book: (a: number, b: number, c: number, d: number) => number;
  readonly upload_pic: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly ring_core_0_17_8_bn_mul_mont: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h69de8b0be872d005: (a: number, b: number, c: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__h1929e75d046aca82: (a: number, b: number, c: number, d: number) => void;
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
