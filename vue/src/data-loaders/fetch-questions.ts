import { QUESTION_LIST_HANDLER_URL, TOKEN_HEADER_NAME, URL_PARAM_TOPIC } from "@/constants";
import { type QuestionWithHistory } from "@/interfaces";

/** Makes the best effort to fetch a list of questions. Returns `undefined` on error. */
export const fetchQuestions = async (topic?: string, token?: string): Promise<QuestionWithHistory[] | undefined> => {
  console.log(`Fetching questions for: ${topic}`);

  if (!topic) {
    console.log("No topic provided - getting user-authored questions.");
  }

  // add a token with the email, if there is one (logged in users)
  const headers = new Headers();
  if (token) headers.append(TOKEN_HEADER_NAME, token);

  // no topic param if no topic is provided
  const fetchParams = topic ? `${URL_PARAM_TOPIC}=${topic}` : "";

  try {
    const response = await fetch(`${QUESTION_LIST_HANDLER_URL}${fetchParams}`,
      {
        headers: headers,
        signal: AbortSignal.timeout(5000),
      });
    console.log(`Fetched. Status: ${response.status}`);

    // a successful response should contain the saved question
    // an error may contain JSON or plain text, depending on where the errror occurred
    if (response.status === 200) {
      try {
       const questions = <QuestionWithHistory[]>await response.json();
        // console.log(questions.value);
        console.log(`Questions loaded: ${questions.length}`);

        return questions;

      } catch (error) {
        console.error(error);
        console.error("Failed to parse list of questions.");
      }
    }
    else {
      console.error("Failed to get questions. Status: ", response.status);
    }
  } catch (error) {
    console.error("Failed to get questions.", error);
  }
};