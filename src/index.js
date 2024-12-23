import * as React from "react";
import * as ReactDOM from "react-dom/client";
import { BrowserRouter } from "react-router-dom";
import { Routes, Route, Link } from "react-router-dom";

// import * as serviceWorker from './serviceWorker';
import App from "./components/app";
import Scan from "./components/scan";
import BookDetails from "./components/bookDetails";
import Welcome from "./components/welcome";
import { AuthLogin } from "./components/authLogin";
import { AuthLogout } from "./components/authLogout";
import { Auth0ProviderWithNavigate } from "./components/auth0-provider-with-navigate";

import ".//css/index.css";

// console.log("app started")

ReactDOM.createRoot(document.getElementById("app")).render(

  <BrowserRouter>
    <Auth0ProviderWithNavigate>
      <Routes>
        <Route path="/" element={<App />}>
          <Route index element={<Welcome />} />
          <Route path="scan" element={<Scan scanRate={250} />} />
          <Route path="about" element={<About />} />
          <Route path="dashboard" element={<Dashboard />} />
          <Route path="login" element={<AuthLogin />} />
          <Route path="logout" element={<AuthLogout />} />
          <Route path="about" element={<About />} />
          <Route path="*" element={<BookDetails />} />
        </Route>
      </Routes>
    </Auth0ProviderWithNavigate>
  </BrowserRouter>
);

// serviceWorker.register();

function About() {
  return (
    <div>
      <p class="mt-8">Bookworm is a free app for book lovers to help use remember and share the books we read.</p>
      <h1 class="mt-8 text-center">Privacy Policy</h1>
      <p class="mt-4">No data from this app is shared with any third party.</p>
      <ul>
        <li>Authentication: <a href="https://auth0.com">Auth0</a></li>
        <li>Infrastructure: AWS</li>
        <li>Source code: <a href="https://github.com/rimutaka/bookwormfood">https://github.com/rimutaka/bookwormfood</a></li>
        <li>Contact and support: <a href="mailto:max@onebro.me">max@onebro.me</a></li>
      </ul>
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

