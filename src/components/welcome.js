import React, { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { build_book_url } from "./scanResult";
import useState from 'react-usestateref';
import initWasmModule, { get_scanned_books } from '../wasm-rust/isbn_mod.js';


export default function Welcome() {

  const navigate = useNavigate();
  const [books, setBooks] = useState([]); // the list of books saved in localStorage

  useEffect(() => {

    // these values are used to set the meta tags in index.html
    // and have to be reset when the component is mounted from
    // a scan that sets them to the book details
    // make sure the values are synchronized with index.html
    // TODO: change ids to constants
    document.title = "📖📚📚";

    // get the list of books from the localStorage
    (async () => {
      await initWasmModule(); // run the wasm initializer before calling wasm methods
      // console.log("Requesting scanned books");
      // request book data from WASM module
      // the responses are sent back as messages to the window object   
      get_scanned_books();
      // console.log("Requested scanned books (inside async)");
    })();

    // console.log("Requested scanned books (outside async)");
  }, []);

  window.addEventListener("message", (msg) => {
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
    if (data?.localBooks?.Ok) {
      let list_of_books = data.localBooks.Ok?.books;
      // console.log(`Books: ${JSON.stringify(list_of_books)}`);
      setBooks(list_of_books);
    }
    else {
      // console.log("Welcome screen received a message that is not a list of books");
      // console.log(data);
    }
  });

  const onBtnClickHandler = async (e) => {
    e.preventDefault();
    navigate(`scan`)
  };

  const renderButtons = () => {
    return <div className="scanBtn">
      <button onClick={onBtnClickHandler}>SCAN barcode</button>
    </div>
  };

  // renders the list of books saved in localStorage
  const renderList = () => {

    const book_list = [];

    books.forEach((book) => {
      let url = build_book_url(book.title, book.author, book.isbn);
      book_list.push(<li key={book.isbn}><a href={url}>{book.title}</a> {" by " + book.author}</li>);
    });

    return <ul className="scanList">
      {book_list}
    </ul>
  };

  const renderWelcomeMsg = () => {
    return <div id="welcomeMsg" className="welcome">
      <div>
        <h1>Scan the book's barcode to learn, record or share</h1>
        <ul>
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
