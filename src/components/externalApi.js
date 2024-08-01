import React, { useState } from 'react';
import { useAuth0 } from '@auth0/auth0-react';

export const ExternalApi = () => {
  const [message, setMessage] = useState('');
  const serverUrl = process.env.REACT_APP_SERVER_URL;

  const { getAccessTokenSilently, getIdTokenClaims } = useAuth0();


  const callSecureApi = async () => {
    try {
      console.log("getting token");
      const token = await getAccessTokenSilently();
      console.log(`token: ${token}`);

      const claims = await getIdTokenClaims();
      // console.log(`claims: ${JSON.stringify(claims)}`);
      console.log(`claims: ${claims.__raw}`);

      if (claims?.__raw) {
        console.log("fetching");
        const response = await fetch(
          "https://bookwormfood.com/sync.html",
          {
            headers: {
              Authorization: `Bearer ${token}`,
              Auth0: `${token}`,
            },
          },
        );
        const responseData = await response?.text();

        console.log(`response: ${responseData}`);

        setMessage(responseData);
      }
    } catch (error) {
      setMessage(error.message);
    }
  };

  return (
    <div className="container">
      <h1>External API</h1>
      <p>
        Use these buttons to call an external API. The protected API call has an
        access token in its authorization header. The API server will validate
        the access token using the Auth0 Audience value.
      </p>
      <div
        className="btn-group mt-5"
        role="group"
        aria-label="External API Requests Examples"
      >
        <button
          type="button"
          className="btn btn-primary"
          onClick={callSecureApi}
        >
          Get Protected Message
        </button>
      </div>
      {message && (
        <div className="mt-5">
          <h6 className="muted">Result</h6>
          <div className="container-fluid">
            <div className="row">
              <code className="col-12 text-light bg-dark p-4">{message}</code>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default ExternalApi;




