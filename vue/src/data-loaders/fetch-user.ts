import { USER_HANDLER_URL, TOKEN_HEADER_NAME } from "@/constants";
import { type User } from "@/interfaces";

/** Makes the best effort to fetch user details. Returns `undefined` on error. */
export const fetchUser = async (email: string, token: string): Promise<User | undefined> => {
  console.log(`Fetching user details for: ${email}`);

  // only fetch if the user is known
  if (!email) return;

  // redirect the user to login with the list of topics as a parameter
  if (!token) {
    console.log("No token found.");
    return; // unreachable code
  }

  try {
    const response = await fetch(USER_HANDLER_URL, {
      headers: {
        [TOKEN_HEADER_NAME]: token,
      },
    });

    console.log(`Fetched. Status: ${response.status}`);

    // a successful response should contain full user details
    // an error may contain JSON or plain text, depending on where the error occurred
    if (response.status === 200) {
      try {
        const user = <User>await response.json();
        console.log("User details fetched");
        // console.log(user);

        // return undefined if the user struct is not complete
        if (!user.email) return;

        return user;

      } catch (error) {
        console.error(error);
      }
    }
    else {
      // only 200 and 404 come from lambda, any other code is an error
      console.error("Failed to get user. Status: ", response.status);
    }
  } catch (error) {
    console.error("Failed to get user.", error);
  }
};