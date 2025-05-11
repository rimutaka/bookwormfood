/// A mirror of the Rust's type
export interface Answer {
  /// Answer option
  a: string,
  /// Detailed explanation
  e: string,
  /// Set to true if correct
  c: boolean,
  /// Set to true if this is the user selection
  sel: boolean,
}

/// A mirror of the Rust's type
export interface Stats {
  correct: number,
  incorrect: number,
  skipped: number,
}

/** Controls visibility of the question.
* - Draft - visible to the author and mods
* - Published - visible to everyone */
export enum PublishStage {
  Draft = "draft",
  Published = "published",
}

/// A mirror of the Rust's type
export interface Question {
  /// Leave blank for new questions
  qid: string,
  /// Required - use the TOPICS.id
  topic: string,
  question: string,
  answers: Array<Answer>,
  /// It is recalculated on the server on submission
  correct: number,
  /// A hash of the email address of the original author
  /// The value submitted by the user is ignored
  readonly author?: string,
  /// When the question was last updated
  updated?: string,
  /// Stats for how the question is used
  stats?: Stats,
  contributor?: ContributorProfile,
  /** A one line summary of the question to use as the title. 
   * Must be plain text. No markdown.
  */
  title: string,
  /** This value is read-only. The server ignores the values submitted from UI. */
  stage: PublishStage,
  /** An ordered list of URLs for refresher material to display on request */
  refresherLinks?: string[],
}

/// A mirror of the Rust's type
export interface User {
  /// User's email address
  email: string,
  /// An SHA256 hash of the email address in hex format
  emailHash: string,
  /// The list of subscribed topics
  topics: string[],
  /// A unique string to use an unsubscribe key
  /// A shortened base58 encoded UUID
  unsubscribe: string,
  /// When the user sub was last updated
  updated: string,
  details?: {
    profile: ContributorProfile,
  },
  /** True if the user is a moderator, otherwise undefined */
  isMod?: boolean,

}

/// Questions contributor details to be displayed alongside the question
export interface ContributorProfile {
  name?: string,
  imgUrl?: string,
  url?: string,
  about?: string,
}

/** Compares all properties so that blank, empty and absent values are treated as equal.
 * Returns true if both are undefined or null.
 * Returns false if only one is undefined or null.
 */
export function CompareContributors(one?: ContributorProfile, other?: ContributorProfile) {
  if (!one && !other) return true;
  if (!one || !other) return false;
  return one.name === other.name && one.imgUrl == other.imgUrl && one.url == other.url && one.about == other.about;
}

/** A set of values to show what the contribution message may look like */
export const CONTRIBUTOR_PLACEHOLDER: ContributorProfile =
{
  name: "Your name",
  url: "https://your-website.com",
  imgUrl: "/your-logo.png",
  about: "Something about you",
};

/// Indicates the status of loading / fetching data
export enum LoadingStatus {
  Loading, // awaiting response
  Loaded, // data found and is loaded into the app
  NoData, // checked the DB, but no data found
  Error // something went wrong
}

/** A list of fields used for question sponsorship payments
 * A mirror of the Rust's type  */
export interface QuestionSponsorship {
  qty: number,
  cancelUrl: string,
  successUrl: string,
  topics?: string,
  contributor?: ContributorProfile,
}

/** A TS implementation of Rust's  AnswerStatus enum
 * where every variant wraps DateTime<Utc> type.
 * E.g. 2024-01-01T00:00:00Z
 */
export interface AnswerStatus {
  asked?: string,
  correct?: string,
  incorrect?: string,
  skipped?: string,
}

/** Mirrors Rust's QuestionWithHistory struct */
export interface QuestionWithHistory {
  question: Question,
  history?: Array<AnswerStatus>,
}