import * as React from "react";
import * as ReactDOM from "react-dom/client";
import { BrowserRouter } from "react-router-dom";
import { Routes, Route, Outlet, Link } from "react-router-dom";

import * as serviceWorker from './serviceWorker';
import Scan from "./components/scan";
import BookDetails from "./components/bookDetails";
import Welcome from "./components/welcome";

import ".//css/index.css";

console.log("app started")

// check https://github.com/rafgraph/spa-github-pages for using GH Pages with BrowserRouter

ReactDOM.createRoot(document.getElementById("app")).render(

  <BrowserRouter>
    <Routes>
      <Route path="/" element={<Layout />}>
        <Route index element={<Welcome />} />
        <Route path="scan" element={<Scan scanRate={250} />} />
        <Route path="about" element={<About />} />
        <Route path="dashboard" element={<Dashboard />} />
        <Route path="*" element={<BookDetails />} />
      </Route>
    </Routes>
  </BrowserRouter>
);

// ReactDom.render((
//   <div className="main">
//     <Scan scanRate={250} covid19={true} upnqr={true} />
//   </div>
// ), document.getElementById("app"));

serviceWorker.register();

function Layout() {
  return (
    <div className="main">
      <Outlet />
    </div>
  );
}

function About() {
  return (
    <div>
      <h2>About</h2>
    </div>
  );
}

function Dashboard() {
  return (
    <div>
      <h2>Dashboard</h2>
    </div>
  );
}

function NoMatch() {
  return (
    <div>
      <h2>Nothing to see here!</h2>
      <p>
        <Link to="/">Go to the home page</Link>
      </p>
    </div>
  );
}