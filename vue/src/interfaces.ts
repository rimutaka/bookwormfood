import { ReadStatus } from "@/wasm-rust/isbn_mod";

/// A mirror of the Rust's type
export interface VolumeInfo {
  description: string | undefined,
}

/// A mirror of the Rust's type
export interface Book {
  isbn: number,
  title: string | undefined,
  authors: string[] | undefined,
  readStatus: ReadStatus,
  cover: string | undefined,
  volumeInfo: VolumeInfo | undefined,
  shareId: number | undefined,
  photos: string[] | undefined,
}

/** Creates a standardised book URL.
 * TODO: move it to WASM
 */
export function buildBookUrl(book: Book, readerId?: string): string {

  const title = book.title || "";
  const authors = book.authors?.[0] || "";

  let url = (authors) ?
    (title.toLowerCase().replace(/[^a-z0-9]/g, "-") + "-by-" + authors.toLowerCase().replace(/[^a-z0-9]/g, "-").replace(/,/g, "") + "/" + book.isbn + "/").replace(/-{2,}/g, "-")
    : (title.toLowerCase().replace(/[^a-z0-9]/g, "-") + "/" + book.isbn + "/").replace(/-{2,}/g, "-");

  // reader id is only present in shared links
  // it is ignored if the reader id is for the current user, but the code does not know that
  // until the server responds with the list of photos
  if (readerId) {
    url = url.replace(/\/$/, "") + "/reader-" + readerId;
  }

  return url;
}