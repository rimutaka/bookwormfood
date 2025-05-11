/// The endpoint for question-related requests.
export const QUESTION_HANDLER_URL = "https://bitesized.info/q?";
export const QUESTION_LIST_HANDLER_URL = "https://bitesized.info/ql?";
export const QUESTION_STAGE_HANDLER_URL = "https://bitesized.info/qs?";
export const QUESTION_FEEDBACK_HANDLER_URL = "https://bitesized.info/qf?";
/// The endpoint for user-related requests.
export const USER_HANDLER_URL = "https://bitesized.info/u?";
/// The endpoint for payment-related requests.
export const PAYMENTS_HANDLER_URL = "https://bitesized.info/checkout?";

/// E.g. .../q?topic=foo&qid=bar
export const URL_PARAM_TOPIC = "topic"
/// E.g. .../q?topics=foo,bar
export const URL_PARAM_TOPICS = "topics"
/// E.g. .../q?topic=foo&qid=bar
export const URL_PARAM_QID = "qid"
/** E.g. .../q?topic=foo&qid=bar&stage=published, must be one of PublishStage enum values  */
export const URL_PARAM_STAGE = "stage"
/// E.g. .../q?topic=foo&qid=bar&answers=0.1
export const URL_PARAM_ANSWERS = "answers"
/// A character used to separate values within the same param value,
/// e.g. .../q?topics=foo.bar
export const URL_PARAM_LIST_SEPARATOR = "."

/// A temporary measure to limit who can save data in DDB
export const TOKEN_HEADER_NAME = "x-bitie-token";

/** x-amz-content-sha256 is required for all PUT/POST requests.
 * See https://docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-header-based-auth.html
 * Empty body value: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
*/
export const AWS_BODY_HASH_HEADER = "x-amz-content-sha256";

/// The key name for the last authentication timestamp in the localStorage.
/// The user is asked to auth if the key is present.
export const LAST_AUTH_TIMESTAMP = "auth";

/// The name of the preview question in the localStorage and the name of the popup window
/// with live preview rendering
export const PREVIEW_QUESTION_LS_KEY = "previewQuestion";

/// A locally cached value of the last contributor details entered by this user
/// for future reuse with new questions
export const CONTRIBUTOR_DETAILS_LS_KEY = "contributor";

/// A locally cached value of the last sponsor details entered by this user
/// for future reuse with new payments
export const SPONSOR_DETAILS_LS_KEY = "sponsorship";

export const MAX_NUMBER_OF_QUESTIONS_PER_PAYMENT = 20;

/// The maximum size of a serialized question in bytes
/// .from_str() returns an error if the size is exceeded.
export const MAX_QUESTION_LEN = 12_000;

/// The maximum size of a deserialized question in bytes
/// The excess is truncated.
/// The lambda may allow for a few extra bytes to account for the Unicode overhead.
export const MAX_TITLE_LEN = 100;

/** Keypair for the topic title and DDB topic ID, e.g. "AWS"/"aws". */
export interface TopicFields {
  t: string,
  id: string,
};

/// The exclusive list of topics that can be used in the app
/// It is sync'd with the DDB and the lambdas manually
export const TOPICS = <Array<TopicFields>>[
  { t: "AWS", id: "aws" },
  { t: "CSS", id: "css" },
  { t: "Essentials", id: "general" },
  { t: "JS / TS", id: "js-ts" },
  { t: "Rust", id: "rust" }
];

/** Returns a random topic id from the TOPICS list  */
export function randomTopicId(): string {
  return TOPICS[Math.floor(Math.random() * TOPICS.length)].id;
}

/** Choose this for random questions if nothing else was selected.
 * It's a temp crutch. TODO: do not allow any data fetching without a topic.
 */
export const DEFAULT_TOPIC = "general";

/** Returns the topic title by its ID */
export function findTopicById(id: string | undefined): string | undefined {
  if (!id) return undefined;
  return TOPICS.find((topic) => topic.id === id)?.t;
}
