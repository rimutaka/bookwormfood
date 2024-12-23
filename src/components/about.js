import React, { useEffect } from "react";

export default function About() {
  return (
    <div>
      <p>Bookworm is a free app for book lovers to help us remember and share the books we read.</p>
      <h1>Privacy Policy</h1>
      <p>No data from this app is shared with any third party.</p>
      <ul>
        <li>Authentication: <a href="https://auth0.com">Auth0</a></li>
        <li>Infrastructure: AWS</li>
        <li>Source code: <a href="https://github.com/rimutaka/bookwormfood">https://github.com/rimutaka/bookwormfood</a></li>
      </ul>
    </div>
  )
};
