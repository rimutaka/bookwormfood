import React from "react";
import useState from 'react-usestateref';
import { useNavigate, useLocation } from "react-router-dom";
import initWasmModule, { get_book_data } from '../wasm-rust/isbn_mod.js';

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
    return <h3 className="fade-in">{text}</h3>
  }
  else {
    return null;
  }
}

// stores the previous ISBN to check against the one in params
// to avoid unnecessary calls to the WASM module
let previousIsbn = "";

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
    if (data?.googleBooks?.Ok) {
      let title = data.googleBooks.Ok?.items[0]?.volumeInfo?.title;
      if (!title) title = "No data in Google for this ISBN code";
      // console.log(`Title: ${title}`);
      setTitle(title);
      let authors = data.googleBooks.Ok?.items[0]?.volumeInfo?.authors?.join(", ");
      setAuthors(authors);
      let thumbnail = data.googleBooks.Ok?.items[0]?.volumeInfo?.imageLinks?.thumbnail;
      setThumbnail(thumbnail);
      // if (thumbnail) setThumbnail(thumbnail);
      // const amount = data.googleBooks.Ok?.items[0]?.saleInfo?.listPrice?.amount;
      // const currency = data.googleBooks.Ok?.items[0]?.saleInfo?.listPrice?.currencyCode;
      // if (amount) setPrice(`${currency} ${amount}`);

      // navigate to the new URL with the book title, e.g. https://localhost:8080/the-subtle-art-of-not-giving-a-f-k-by-mark-manson/9780062457714/
      let url = (title.toLowerCase().replace(/[^a-z0-9]/g, "-") + "-by-" + authors.toLowerCase().replace(/[^a-z0-9]/g, "-").replace(/,/g, "") + "/" + isbn + "/").replace(/-{2,}/g, "-");
      navigate(`/${url}`);
    }
    else {
      // console.log(data);
      setTitle("Cannot get data from Google for this book");
    }
  });

  const renderQrCodeResult = () => {

    // update page title
    let fullTitle = (title) ? title + " by " + authors : "Book not found";
    document.title = fullTitle;

    // update meta tags for social media sharing
    // TODO: change ids to constants
    document.getElementById("ogImage").setAttribute('content', thumbnail);
    document.getElementById("ogTitle").setAttribute('content', fullTitle);

    return (
      <div>
        <p>ISBN: {isbn}</p>
        <HtmlH3 text={title} />
        <HtmlP text={`by ${authors}`} />
        <table>
          <tr>
            <th>About</th>
            <th>Buy</th>
            <th>Borrow</th>
          </tr>
          <tr>
            <td>
              <p><a href={`https://www.goodreads.com/search?q=${isbn}`}>GoodReads</a></p>
              <p><a href={`https://app.thestorygraph.com/browse?search_term=${isbn}`}>StoryGraph</a></p>
              <p><a href={`https://www.google.com/search?tbo=p&tbm=bks&q=isbn:${isbn}`}>Google books</a></p>
            </td>
            <td>
              <p><a href={`https://www.thenile.co.nz/search?s.q=${isbn}`}>The Nile</a></p>
              <p><a href={`https://www.amazon.com/s?k=${isbn}`}>Amazon</a></p>
              <p><a href={`https://www.mightyape.co.nz/books?q=${isbn}`}>MightyApe</a></p>
            </td>
            <td>
              <p><a href={`https://discover.aucklandlibraries.govt.nz/search?query=${isbn}`}>Auckland libraries</a></p>
            </td>
          </tr>
        </table>
      </div>)
  }
  const onClickBackHandler = (e) => {
    e.preventDefault();
    navigate(`/scan`)
  };

  const onClickCopyToClipboard = async (e) => {
    e.preventDefault();
    await navigator.clipboard.writeText(window.location.href);
    const btnId = document.getElementById("copyToClip");
    btnId.innerText = "COPIED TO CLIPBOARD";
    btnId.style.backgroundColor = "green";
    setTimeout(() => {
      btnId.innerText = "SHARE";
      btnId.style.backgroundColor = "";
    }, 3000);
  }

  const renderCopyToClipboardBtn = () => {
    return <a href="!#" style={{ padding: 12 }} id="copyToClip" className="myHref"
      onClick={onClickCopyToClipboard}>SHARE</a>
  }

  const renderResult = () => {
    return (
      <div className="resultModal">
        <div className="result">
          {renderQrCodeResult()}
        </div>
        <div style={{ marginTop: 40 }}>
          <a href="!#" style={{ padding: 12 }} className="myHref" onClick={onClickBackHandler}>SCAN AGAIN</a>
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
