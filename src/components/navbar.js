import React, { useEffect } from "react";
import { useAuth0 } from "@auth0/auth0-react";

export default function Navbar() {

  const { isAuthenticated } = useAuth0();

  useEffect(() => {
    // toggle sign in / sign out button
  }, [isAuthenticated]);

  const loginButton = () => {
    const { loginWithRedirect } = useAuth0();

    const handleLogin = async () => {
      await loginWithRedirect({
        appState: {
          returnTo: window.location.pathname,
        },
      });
    };

    return (
      <div>
        <button className="loginBtn" onClick={handleLogin} type="button" title="Login to save your books to in the cloud" >
          <svg className="mr-2 -ml-1 w-4 h-4" aria-hidden="true" focusable="false" data-prefix="fab" data-icon="google" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 488 512"><path fill="currentColor" d="M488 261.8C488 403.3 391.1 504 248 504 110.8 504 0 393.2 0 256S110.8 8 248 8c66.8 0 123 24.5 166.3 64.9l-67.5 64.9C258.5 52.6 94.3 116.6 94.3 256c0 86.5 69.1 156.6 153.7 156.6 98.2 0 135-70.4 140.8-106.9H248v-85.3h236.1c2.3 12.7 3.9 24.9 3.9 41.4z"></path></svg>
          Sign in with Google
        </button>
      </div>
    );
  };

  const logoutButton = () => {
    const { logout } = useAuth0();

    const handleLogout = async () => {
      await logout({
        logoutParams: { returnTo: "https://" + window.location.hostname + (window.location.port == 80 ? "" : ":" + window.location.port) + "/logout" }
      });
    };

    return (
      <div>
        <button className="loginBtn" onClick={handleLogout} type="button" >Sign out
        </button>
      </div>
    );
  };

  // tidy up the app version that comes from an env var set in package.json for START and BUILD
  // example 2024-08-25 15:43:14+12:00
  let appVersion = process.env.REACT_APP_BUILD_TS;
  appVersion = appVersion.substring(5, 6) == "0"
    ? appVersion.substring(6, 16) // do not show leading zero for 08-25
    : appVersion.substring(5, 16);
  // convert it to v.825.1608
  appVersion = "v." + appVersion.replace("-", "").replace(":", "").replace(" ", ".");


  return (
    <nav className="mx-auto py-4 border-t mt-12 text-xs">
      <ul className="flex flex-wrap font-small">
        <li className="flex-none">Bookworm App</li>
        <li className="flex-grow text-center"><span className="align-baseline">{appVersion}</span></li>
        <li className="flex-none">
          {isAuthenticated ? logoutButton() : loginButton()}
        </li>
      </ul>
    </nav>
  );
};
