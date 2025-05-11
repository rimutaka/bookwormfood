import React, { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { useAuth0 } from "@auth0/auth0-react";
import { build_book_url } from "./bookDetails.js";
import useState from 'react-usestateref';
import initWasmModule, { get_scanned_books, ReadStatus } from '../wasm-rust/isbn_mod.js';

// Books should be fetched from the cloud only once.
// The local storage is expected to be in sync while the app is active.
// A reload resets the flag.
// true: fetch books from the cloud, false: already fetched
let withCloudSync = true;

export default function Welcome() {

  const navigate = useNavigate();
  const [books, setBooks] = useState([]); // the list of books saved in localStorage
  const { isAuthenticated, getIdTokenClaims } = useAuth0();
  const [token, setToken] = useState();

  const handleWasmMessage = (msg) => {
    // console.log(`WASM msg: ${msg.data} / ${msg.origin} / ${msg.source}`);
    // WASM messages should be JSON objects
    let data;
    try {
      data = JSON.parse(msg.data);
    }
    catch (e) {
      // use this log for debugging, but this mostly logs messages sent from React tooling
      // in development mode, not sure it's worth logging this in production
      // console.log(`Error parsing JSON data: ${e}`);
      return;
    }

    // see `WasmResult` and `WasmResponse` in the WASM code for the structure of the data
    if (data?.localBooks?.Ok?.books) {
      let list_of_books = data.localBooks.Ok?.books;
      // console.log(`Books: ${JSON.stringify(list_of_books)}`);
      setBooks(list_of_books);
    }
    else {
      // console.log("Welcome screen received a message that is not a list of books");
      // console.log(data);
    }
  };


  useEffect(() => {
    // handles messages with book data sent back by the WASM module
    window.addEventListener("message", handleWasmMessage);

    // these values are used to set the meta tags in index.html
    // and have to be reset when the component is mounted from
    // a scan that sets them to the book details
    // make sure the values are synchronized with index.html
    // TODO: change ids to constants
    document.title = "📖📚📚";

    // get the list of books from the localStorage
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

      await initWasmModule(); // run the wasm initializer before calling wasm methods
      // console.log("Requesting scanned books");
      // request book data from WASM module
      // the responses are sent back as messages to the window object 
      // console.log(`Read token: ${idTokenClaims?.__raw}, sync: ${withCloudSync}`);
      get_scanned_books(idTokenClaims?.__raw, withCloudSync);
      // prevent future list syncs until the page is refreshed
      if (idTokenClaims?.__raw) withCloudSync = false;
      // console.log("Requested scanned books (inside async)");
    })();

    // console.log("Requested scanned books (outside async)");

    // remove the listener to avoid adding it multiple times
    return () => window.removeEventListener("message", handleWasmMessage);
  }, [isAuthenticated]);

  const onScanBtnClickHandler = async (e) => {
    e.preventDefault();
    navigate(`scan`)
  };

  const onBookLinkClickHandler = async (e) => {
    e.preventDefault();
    const path = e.target.getAttribute('data-url');
    navigate(path);
  };

  const renderButtons = () => {
    return <div className="scanBtn">
      <button onClick={onScanBtnClickHandler}>SCAN barcode</button>
    </div>
  };

  // renders the list of books saved in localStorage
  const renderList = () => {

    const book_list = [];

    books.forEach((book) => {

      // choose the right status icon
      if (book.title) {
        // default is a blank space
        let statusIcon = "blank";
        switch (book.readStatus) {
          case ReadStatus[0]:
            statusIcon = "icon-alarm";
            break;
          case ReadStatus[1]:
            statusIcon = "icon-checkmark";
            break;
          case ReadStatus[2]:
            statusIcon = "icon-heart";
            break;
        }

        let url = build_book_url(book.title, book.authors?.[0], book.isbn);
        book_list.push(<li key={book.isbn}>
          <i className={statusIcon}></i>
          <a href={url} data-url={url} onClick={onBookLinkClickHandler}>{book.title}</a>
          {book.authors ? " by " + book.authors?.[0] : ""}
        </li>);
      }
    });

    return <ul className="scan-list">
      {book_list}
    </ul>
  };

  const renderWelcomeMsg = () => {
    return <div id="welcomeMsg" className="welcome">
      <div>
        <h1>Scan the book's barcode to learn, record or share</h1>
        <ul className={books.length > 4 ? "hidden" : ""}>
          <li>View reviews, book and author details</li>
          <li>Borrow from Auckland Libraries</li>
          <li>Buy new or secondhand</li>
          <li>Save it in your reading list</li>
        </ul>
      </div>
    </div>;
  };

  return (
    <div>
      {renderWelcomeMsg()}
      {renderButtons()}
      {renderList()}
    </div>
  )
};
