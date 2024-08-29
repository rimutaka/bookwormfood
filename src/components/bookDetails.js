import React, { useEffect } from "react";
import useState from 'react-usestateref';
import { useNavigate, useLocation } from "react-router-dom";
import { useAuth0 } from '@auth0/auth0-react'
import initWasmModule, { get_book_data, ReadStatus, update_book_status, delete_book, upload_pic } from '../wasm-rust/isbn_mod.js';

function HtmlP({ text, classNames }) {

  if (text && !text.includes("undefined")) {
    return <p className={"fade-in " + classNames ?? ""}>{text}</p>
  }
  else {
    return null;
  }
}

function HtmlH3({ text }) {

  if (text) {
    return <h3 className="fade-in font-bold">{text}</h3>
  }
  else {
    return null;
  }
}

// An expandable book description. Expands once and does not collapse back.
// Shorter descriptions are displayed in full.
function HtmlDescription({ text }) {
  const classNames = "fade-in py-2 text-xs max-w-prose"

  if (text && !text.includes("undefined")) {
    // make long passages collapsible
    if (text.length > 500) {
      return <p className={classNames}>
        {text.substring(0, 200)}
        <span className="descr-collapsed">
          <span className="descr-expand" onClick={(e) => e.target.parentElement.className = "descr-full"}>more</span>
          <span className="descr-extra-text">{text.substring(200)}</span>
        </span>
      </p>
    }
    else {
      // short enough to be displayed in full
      return <p className={classNames}>{text}</p>
    }
  }
  else {
    return null;
  }
}

export function build_book_url(title, authors, isbn) {
  if (authors) {
    return (title.toLowerCase().replace(/[^a-z0-9]/g, "-") + "-by-" + authors.toLowerCase().replace(/[^a-z0-9]/g, "-").replace(/,/g, "") + "/" + isbn + "/").replace(/-{2,}/g, "-");
  }
  else {
    return (title.toLowerCase().replace(/[^a-z0-9]/g, "-") + "/" + isbn + "/").replace(/-{2,}/g, "-");
  }
}

// The key name for the last authentication timestamp in the localStorage
export const LAST_AUTH_TIMESTAMP = "auth";

export default function BookDetails() {

  const navigate = useNavigate();
  const location = useLocation();
  const { getIdTokenClaims, isAuthenticated } = useAuth0();

  // console.log(location);
  // the ISBN can be located anywhere in the URL, 
  // e.g. https://localhost:8080/the-subtle-art-of-not-giving-a-f-k-by-mark-manson/9780062457714/
  // or https://localhost:8080/9780062457714/the-subtle-art-of-not-giving-a-f-k-by-mark-manson
  let isbn = location.pathname.match(/\/\d{13}(\/|$)/)?.[0]?.replace(/\//g, "") || "";
  // console.log(`ISBN: ${isbn}`);

  const [title, setTitle] = useState();
  const [authors, setAuthors] = useState();
  // const [price, setPrice] = useState();
  const [cover, setCover] = useState();
  const [status, setStatus] = useState();
  const [description, setDescription] = useState();
  const [token, setToken] = useState();
  const [selectedFile, setSelectedFile] = useState(null);

  // Handles different types of responses sent from the WASM module
  // asynchronously. E.g. Book, Books, with the result of the operation
  const handleWasmMessage = (msg) => {
    // console.log(`WASM msg: ${msg.data} / ${msg.origin} / ${msg.source}`);

    // WASM messages should be JSON objects
    let data;
    try {
      data = JSON.parse(msg.data);
    }
    catch (e) {
      // use this for debugging, but this mostly logs messages sent from React tooling
      // in development mode, not sure it's worth logging this in production
      // console.log(`Error parsing JSON data: ${e}`);
      return;
    }

    // see `WasmResult` and `WasmResponse` in the WASM code for the structure of the data
    if (data?.localBook?.Ok) {
      const book = data.localBook.Ok;
      let title = book.title;
      if (!title) title = "No data in Google for this ISBN code";
      // console.log(`Title: ${title}`);
      setTitle(title);
      let authors = book.authors?.join(", ");
      setAuthors(authors);
      let cover = book.cover;
      setCover(cover);
      let status = book.readStatus;
      setStatus(status);
      let description = book.volumeInfo?.description;
      setDescription(description);
      // if (thumbnail) setThumbnail(thumbnail);
      // const amount = data.googleBooks.Ok?.items[0]?.saleInfo?.listPrice?.amount;
      // const currency = data.googleBooks.Ok?.items[0]?.saleInfo?.listPrice?.currencyCode;
      // if (amount) setPrice(`${currency} ${amount}`);

      // navigate to the new URL with the book title, e.g. https://localhost:8080/the-subtle-art-of-not-giving-a-f-k-by-mark-manson/9780062457714/
      let url = build_book_url(title, authors, isbn);
      navigate(`/${url}`, { replace: true });
    }
    else if (data?.deleted?.Ok) {
      console.log("Book deletion confirmed");
      navigate(`/`);
    }
    else if (data?.uploaded?.Ok) {
      console.log("File uploaded:", data.uploaded.Ok);
    }
    else {
      // console.log(data);
      setTitle("Cannot get data from Google for this book");
    }
  }

  useEffect(() => {
    // handles messages with book data sent back by the WASM module
    window.addEventListener("message", handleWasmMessage);

    // fetch book data if the ISBN code is found in the URL
    if (isbn) {
      (async () => {
        // try to get the token
        let idTokenClaims = null;
        if (isAuthenticated) {
          idTokenClaims = await getIdTokenClaims();
          if (idTokenClaims?.__raw) {
            setToken(idTokenClaims.__raw);
            // console.log(`JWT: ${idTokenClaims?.__raw}`);
            // console.log(`Expiry: ${idTokenClaims?.exp}`);
          } else {
            console.log(`Missing token: ${JSON.stringify(idTokenClaims)}`);
          }
        } else {
          console.log("User is not authenticated");
        }

        // get book details
        await initWasmModule(); // run the wasm initializer before calling wasm methods
        // request book data from WASM module
        // the responses are sent back as messages to the window object
        // have to use the local for the token because `token` var is not updated in time
        // console.log(`Read token: ${idTokenClaims?.__raw}`);
        get_book_data(isbn, idTokenClaims?.__raw);
      })();
    } else {
      isbn = "no ISBN code found in the URL";
    }

    // remove the listener to avoid adding it multiple times
    return () => window.removeEventListener("message", handleWasmMessage);
  }, [isAuthenticated]);

  const onClickStatusToRead = (e) => {
    update_book_status(isbn, status == ReadStatus[0] ? null : ReadStatus.ToRead, token);
  };

  const onClickStatusRead = (e) => {
    update_book_status(isbn, status == ReadStatus[1] ? null : ReadStatus.Read, token);
  };

  const onClickStatusLiked = (e) => {
    update_book_status(isbn, status == ReadStatus[2] ? null : ReadStatus.Liked, token);
  };

  const onClickStatusBin = (e) => {
    e.preventDefault();
    delete_book(isbn, token);
  };

  const renderQrCodeResult = () => {
    // update page title
    let fullTitle = (title) ? title + " by " + authors : "Book not found";
    document.title = fullTitle;

    return (
      <div>
        <div>
          <HtmlH3 text={title} />
          <HtmlP text={`by ${authors}`} />
          <HtmlDescription text={description} />
          <p className="py-2 text-xs">ISBN: {isbn}</p>
        </div>
        <div className="book-actions">
          <i title="Read later" id="status-later" className={"icon-alarm" + (status == ReadStatus[0] ? " active" : "")} onClick={onClickStatusToRead}></i>
          <i title="Done reading it" id="status-read" className={"icon-checkmark" + (status == ReadStatus[1] ? " active" : "")} onClick={onClickStatusRead}></i>
          <i title="Liked it!" id="status-liked" className={"icon-heart" + (status == ReadStatus[2] ? " active" : "")} onClick={onClickStatusLiked}></i>
          <span className="grow"></span>
          <i title="Bin it" id="status-bin" className="icon-bin text-slate-500" onClick={onClickStatusBin}></i>
        </div>
        <div className="result-table">
          <div>
            <h3 className="about">About</h3>
            <p><a href={`https://www.goodreads.com/search?q=${isbn}`}>GoodReads</a></p>
            <p><a href={`https://app.thestorygraph.com/browse?search_term=${isbn}`}>StoryGraph</a></p>
            <p><a href={`https://www.google.com/search?tbo=p&tbm=bks&q=isbn:${isbn}`}>Google books</a></p>
          </div>
          <div>
            <h3 className="buy">Buy</h3>
            <p><a href={`https://www.thenile.co.nz/search?s.q=${isbn}`}>The Nile</a></p>
            <p><a href={`https://www.amazon.com/s?k=${isbn}`}>Amazon</a></p>
            <p><a href={`https://www.mightyape.co.nz/books?q=${isbn}`}>MightyApe</a></p>
          </div>
          <div>
            <h3 className="borrow">Borrow</h3>
            <p><a href={`https://discover.aucklandlibraries.govt.nz/search?query=${isbn}`}>Auckland libraries</a></p>
          </div>
        </div>
      </div>)
  }

  const renderCoverImage = () => {
    if (cover) {
      return <div className="book-cover fade-in"><img src={cover} alt="Book cover" /></div>
    }
  }

  const onClickMyBooks = (e) => {
    e.preventDefault();
    navigate(`/`)
  };

  const onClickBackHandler = (e) => {
    e.preventDefault();
    navigate(`/scan`)
  };

  const onClickCopyToClipboard = async (e) => {
    e.preventDefault();
    await navigator.clipboard.writeText(window.location.href);
    const btnId = document.getElementById("copyToClip");
    btnId.innerText = "COPIED TO CLIPBOARD";
    btnId.classList.add("done");
    setTimeout(() => {
      btnId.innerText = "SHARE";
      btnId.classList.remove("done");;
    }, 3000);
  }

  const handleFileChange = (event) => {
    const files = event.target.files;
    console.log("Selected file:", files);
    setSelectedFile(files);

    // exit if no file selected
    if (!files || !files.length) {
      console.log("No file selected.");
      return;
    }

    try {
      console.log(`Uploading file: ${files[0]?.name}`);
      upload_pic(isbn, files, token);
      console.log("File queued for uploading");
    } catch (error) {
      console.error("Error queuing file for uploading:", error);
    }
  };

  const renderResult = () => {
    return (
      <div className="resultModal">
        <div className="result">
          {renderQrCodeResult()}
        </div>
        <div className="scanBtn">
          <button onClick={onClickMyBooks}>MY BOOKS</button>
          <button onClick={onClickBackHandler}>SCAN AGAIN</button>
          <button id="copyToClip" onClick={onClickCopyToClipboard}>SHARE</button>
        </div>
        {renderCoverImage()}
        <div className="justify-center">
          {selectedFile?.[0]?.name ? selectedFile?.[0]?.name : ""}
        </div>
        <div className="scanBtn">
          <label htmlFor="idPicUploader" className="cursor-pointer border-2 p-2 rounded-md">Upload photo</label>
          <input id="idPicUploader" name="idPicUploader" className="visuallyhidden" type="file" accept="image/jpeg" onChange={handleFileChange} />
        </div>
      </div>);
  };

  return (
    <div>
      {renderResult()}
    </div>
  )
};
