import { QUESTION_HANDLER_URL, TOKEN_HEADER_NAME, URL_PARAM_TOPIC, URL_PARAM_QID } from "@/constants";
import { type Question } from "@/interfaces";


/** Makes the best effort to fetch a single question. Returns `undefined` on error. */
export const fetchQuestion = async (topic: string, qid?: string, token?: string): Promise<Question | undefined | null> => {

  console.log(`Fetching question for: ${topic} / ${qid}`);

  if (!topic) {
    console.error("No topic provided. It's a bug. Exiting.");
    return undefined;
  }

  // add a token with the email, if there is one (logged in users)
  const headers = new Headers();
  if (token) headers.append(TOKEN_HEADER_NAME, token);

  try {
    // fetching by topic returns a random question
    // fetching with qid returns a specific question
    // fetching any topic has "any" for the topic
    const fetchParams = `${URL_PARAM_TOPIC}=${topic}`.concat(qid ? `&${URL_PARAM_QID}=${qid}` : "");
    console.log("fetch params", fetchParams);

    const response = await fetch(`${QUESTION_HANDLER_URL}${fetchParams}`,
      {
        headers: headers,
        signal: AbortSignal.timeout(5000),
      });
    console.log(`Fetched. Status: ${response.status}`);

    // a successful response should contain the saved question
    // an error may contain JSON or plain text, depending on where the errror occurred
    if (response.status === 200) {
      try {
        const question = <Question>await response.json();
        // console.log(question);
        console.log(`Loaded ${question.topic} / ${question.qid}`);
        return question;
      } catch (error) {
        console.error("Failed to parse question.", error);
      }
    }
    else if (response.status === 404) {
      return null;
    }
    else {
      console.error("Failed to get question. Status: ", response.status);
    }
  } catch (error) {
    console.error("Failed to fetch question.", error);
  }
};

/** Fetches the question in Markdown form for editing. Returns `undefined` on error. */
export const fetchQuestionMD = async (topic: string, qid: string, token: string): Promise<Question | undefined | null> => {

  // fetching an existing question for editing
  console.log(`Fetching question in Markdown for ${topic}/${qid}`);
  if (!token) {
    console.log("No token found.");
    return undefined;
  }

  try {
    const response = await fetch(`${QUESTION_HANDLER_URL}${URL_PARAM_TOPIC}=${topic}&${URL_PARAM_QID}=${qid}`, {
      method: "PUT",
      headers: {
        "x-amz-content-sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855", // empty body hash
        [TOKEN_HEADER_NAME]: token,
      },
    });

    // a successful response should contain the requested question with all fields in Markdown
    // an error may contain JSON or plain text, depending on where the error occurred
    if (response.status === 200) {
      try {
        console.log(`Fetched. Status: ${response.status}`);
        const fetchedQuestion = <Question>await response.json();
        // console.log(question);
        return fetchedQuestion;
      } catch (error) {
        console.error(error);
        return undefined;
      }
    } else {
      console.error("Failed to get question. Status: ", response.status);
      return undefined;
    }
  } catch (error) {
    console.error("Failed to get question.", error);
    return undefined;
  }
};
