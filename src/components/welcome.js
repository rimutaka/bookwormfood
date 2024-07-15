import React, { useEffect } from "react";
import { useNavigate } from "react-router-dom";


export default function Welcome() {

  const navigate = useNavigate();

  useEffect(() => {

    // these values are used to set the meta tags in index.html
    // and have to be reset when the component is mounted from
    // a scan that sets them to the book details
    // make sure the values are synchronized with index.html
    // TODO: change ids to constants
    document.title = "📖📚📚"
  }, []);

  const onBtnClickHandler = async (e) => {
    e.preventDefault();
    navigate(`scan`)
  };

  const renderButtons = () => {
    return <div className="scanBtn">
      <button onClick={onBtnClickHandler}>SCAN barcode</button>
    </div>
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
    </div>
  )
};
