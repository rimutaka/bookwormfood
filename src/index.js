import * as React from "react";
import * as ReactDOM from "react-dom/client";
import { BrowserRouter } from "react-router-dom";
import { Routes, Route, Outlet, Link, useLocation } from "react-router-dom";
import { useAuth0 } from "@auth0/auth0-react";
import { LAST_AUTH_TIMESTAMP } from "./components/bookDetails.js";

import * as serviceWorker from './serviceWorker';
import Scan from "./components/scan";
import BookDetails from "./components/bookDetails";
import Welcome from "./components/welcome";
import { AuthLogin } from "./components/authLogin";
import { AuthLogout } from "./components/authLogout";
import { Auth0ProviderWithNavigate } from "./components/auth0-provider-with-navigate";

import ".//css/index.css";

console.log("app started")

ReactDOM.createRoot(document.getElementById("app")).render(

  <BrowserRouter>
    <Auth0ProviderWithNavigate>
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route index element={<Welcome />} />
          <Route path="scan" element={<Scan scanRate={250} />} />
          <Route path="about" element={<About />} />
          <Route path="dashboard" element={<Dashboard />} />
          <Route path="login" element={<AuthLogin />} />
          <Route path="logout" element={<AuthLogout />} />
          <Route path="*" element={<BookDetails />} />
        </Route>
      </Routes>
    </Auth0ProviderWithNavigate>
  </BrowserRouter>
);

serviceWorker.register();

function Layout() {

  const location = useLocation();
  const { isAuthenticated, isLoading } = useAuth0();

  React.useEffect(() => {

    console.log(`Layout load/auth: ${isLoading}/${isAuthenticated}`);

    // save auth details in the localStorage
    if (!isLoading && isAuthenticated) {
      localStorage.setItem(LAST_AUTH_TIMESTAMP, Date.now());
      console.log("Auth status updated");
    }

  }, [isAuthenticated]);

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

