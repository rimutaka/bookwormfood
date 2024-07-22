import React from "react";
import useState from 'react-usestateref';
import { useNavigate, useLocation } from "react-router-dom";
import initWasmModule, { get_book_data, BookStatus, update_book_status } from '../wasm-rust/isbn_mod.js';

function HtmlP({ text }) {

  if (text && !text.includes("undefined")) {
    return <p className="fade-in">{text}</p>
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

// stores the previous ISBN to check against the one in params
// to avoid unnecessary calls to the WASM module
let previousIsbn = "";

export function build_book_url(title, authors, isbn) {
  if (authors) {
    return (title.toLowerCase().replace(/[^a-z0-9]/g, "-") + "-by-" + authors.toLowerCase().replace(/[^a-z0-9]/g, "-").replace(/,/g, "") + "/" + isbn + "/").replace(/-{2,}/g, "-");
  }
  else {
    return (title.toLowerCase().replace(/[^a-z0-9]/g, "-") + "/" + isbn + "/").replace(/-{2,}/g, "-");
  }
}

export default function ScanResult() {

  const navigate = useNavigate();
  const location = useLocation();

  // console.log(location);
  // the ISBN can be located anywhere in the URL, 
  // e.g. https://localhost:8080/the-subtle-art-of-not-giving-a-f-k-by-mark-manson/9780062457714/
  // or https://localhost:8080/9780062457714/the-subtle-art-of-not-giving-a-f-k-by-mark-manson
  let isbn = location.pathname.match(/\/\d{13}(\/|$)/)?.[0]?.replace(/\//g, "") || "";
  // console.log(`ISBN: ${isbn}`);

  const [title, setTitle] = useState();
  const [authors, setAuthors] = useState();
  // const [price, setPrice] = useState();
  const [thumbnail, setThumbnail] = useState();
  const [status, setStatus] = useState();

  // console.log("render");

  // if the ISBN is different from the previous one, fetch book data
  // from multiple sources with WASM
  if (isbn && isbn !== previousIsbn) {
    previousIsbn = isbn;
    (async () => {
      await initWasmModule(); // run the wasm initializer before calling wasm methods
      // request book data from WASM module
      // the responses are sent back as messages to the window object   
      get_book_data(isbn);
    })();
  } else if (!isbn) {
    isbn = "no ISBN code found in the URL";
  }

  // useEffect(() => { }, []);

  // handles messages with book data sent back by the WASM module
  window.addEventListener("message", (msg) => {
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
      let title = book.volumeInfo.title;
      if (!title) title = "No data in Google for this ISBN code";
      // console.log(`Title: ${title}`);
      setTitle(title);
      let authors = book.volumeInfo.authors?.join(", ");
      setAuthors(authors);
      let thumbnail = book.volumeInfo.imageLinks?.thumbnail;
      setThumbnail(thumbnail);
      let status = book.status;
      setStatus(status);
      // if (thumbnail) setThumbnail(thumbnail);
      // const amount = data.googleBooks.Ok?.items[0]?.saleInfo?.listPrice?.amount;
      // const currency = data.googleBooks.Ok?.items[0]?.saleInfo?.listPrice?.currencyCode;
      // if (amount) setPrice(`${currency} ${amount}`);

      // navigate to the new URL with the book title, e.g. https://localhost:8080/the-subtle-art-of-not-giving-a-f-k-by-mark-manson/9780062457714/
      let url = build_book_url(title, authors, isbn);
      navigate(`/${url}`);
    }
    else {
      // console.log(data);
      setTitle("Cannot get data from Google for this book");
    }
  });



  const onClickStatusToRead = (e) => {
    e.preventDefault();
    update_book_status(isbn, BookStatus.ToRead);
  };

  const onClickStatusRead = (e) => {
    e.preventDefault();
    update_book_status(isbn, BookStatus.Read);
  };

  const onClickStatusLiked = (e) => {
    e.preventDefault();
    update_book_status(isbn, BookStatus.Liked);
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
          <small className="py-2">ISBN: {isbn}</small>
        </div>
        <div className="book-actions">
          <i title="Read later" id="status-later" className={"icon-alarm" + (status == BookStatus[0] ? " active" : "")} onClick={onClickStatusToRead}></i>
          <i title="Done reading it" id="status-read" className={"icon-checkmark" + (status == BookStatus[1] ? " active" : "")} onClick={onClickStatusRead}></i>
          <i title="Liked it!" id="status-liked" className={"icon-heart" + (status == BookStatus[2] ? " active" : "")} onClick={onClickStatusLiked}></i>
          <i title="Bin it" id="status-bin" className="icon-bin text-slate-500"></i>
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

  const renderCopyToClipboardBtn = () => {
    return <button id="copyToClip" onClick={onClickCopyToClipboard}>SHARE</button>
  }

  const renderResult = () => {
    return (
      <div className="resultModal">
        <div className="result">
          {renderQrCodeResult()}
        </div>
        <div className="scanBtn">
          <button onClick={onClickMyBooks}>MY BOOKS</button>
          <button onClick={onClickBackHandler}>SCAN AGAIN</button>
          {renderCopyToClipboardBtn()}
        </div>
      </div>);
  };

  return (
    <div>
      {renderResult()}
    </div>
  )
};
