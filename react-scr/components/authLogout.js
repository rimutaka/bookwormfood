import React, { useEffect } from "react";
import { useAuth0 } from "@auth0/auth0-react";
import { useNavigate } from "react-router-dom";
import { LAST_AUTH_TIMESTAMP } from "./bookDetails.js";

export const AuthLogout = () => {

  const { isAuthenticated, isLoading } = useAuth0();
  const navigate = useNavigate();

  useEffect(() => {
    console.log(`Logout load/auth: ${isLoading}/${isAuthenticated}`);

    if (!isLoading && !isAuthenticated) {
      // remove login timestamp from localStorage if we are sure the user is logged out
      // logged out users are not automatically logged in by other components
      localStorage.removeItem(LAST_AUTH_TIMESTAMP)
      console.log("Logout status updated in LS");
    } else if (!isLoading && isAuthenticated) {
      // this may happen if the user navigated to this page directly, but I am not sure
      console.log("User is authenticated after logout - it's a bug");
    }

    // navigate away only after the auth was resolved
    if (!isLoading) {
      navigate("/");
    }
  }, [isLoading]);

  const onHomeClickHandler = async (e) => {
    e.preventDefault();
    navigate("/");
  };

  return (
    <div>Logout page. Click through to the <a href="/" onClick={onHomeClickHandler}>homepage</a> if you are not redirected automatically within a few seconds.</div>
  );

};