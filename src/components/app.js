import { useEffect } from "react";
import { useLocation, Outlet } from "react-router-dom";
import { useAuth0 } from '@auth0/auth0-react'
import { LAST_AUTH_TIMESTAMP } from "./bookDetails.js";

export default function App() {

  const location = useLocation();
  const { isAuthenticated, isLoading, loginWithRedirect, getAccessTokenSilently } = useAuth0();

  useEffect(() => {

    console.log(`App load/auth: ${isLoading}/${isAuthenticated}`);

    // save auth details in the localStorage
    if (!isLoading) {
      if (isAuthenticated) {
        localStorage.setItem(LAST_AUTH_TIMESTAMP, Date.now());
        console.log("Auth status updated in LS");
      }
      else {
        console.log("Not authenticated");

        (async () => {

          // log in the user if was logged in before
          const lastAuth = localStorage.getItem(LAST_AUTH_TIMESTAMP);
          console.log(`Last auth/auth'd: ${lastAuth}/${isAuthenticated}`);
          if (lastAuth && !isAuthenticated) {

            console.log("User was logged in before, logging in again");
            try {
              const accessToken = await getAccessTokenSilently();
              console.log(`Access token: ${accessToken}`);
            } catch (e) {
              console.log(`Error getting access token: ${e}`);
              await loginWithRedirect({
                appState: {
                  returnTo: window.location.pathname,
                },
              })
            }
          }
        })();
      }
    }

  }, [isLoading, isAuthenticated]);

  return (
    <div className="main">
      <Outlet />
    </div>
  );
};
