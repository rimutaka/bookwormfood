import { Auth0Provider } from "@auth0/auth0-react";
import React from "react";
import { useNavigate } from "react-router-dom";

export const Auth0ProviderWithNavigate = ({ children }) => {
  const navigate = useNavigate();

  const auth0Domain = "auth.bookworm.im"; // "bookwormfood.us.auth0.com";
  const clientId = "nqzjY0VWUu8GoDVbqyfy2yOdgkydrEaf";
  const redirectUri = "https://" + window.location.hostname + (window.location.port == 80 ? "" : ":" + window.location.port) + "/login";
  // console.log(`Redirect URI: ${redirectUri}`);

  const onRedirectCallback = (appState) => {
    navigate(appState?.returnTo || window.location.pathname);
  };

  if (!(auth0Domain && clientId && redirectUri)) {
    return null;
  }

  return (
    <Auth0Provider
      domain={auth0Domain}
      clientId={clientId}
      authorizationParams={{
        redirect_uri: redirectUri,
      }}
      onRedirectCallback={onRedirectCallback}
    >
      {children}
    </Auth0Provider>
  );
};