import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { useRoute } from 'vue-router'

import { CONTRIBUTOR_DETAILS_LS_KEY, URL_PARAM_TOPIC, randomTopicId } from './constants'
import type { Question, QuestionWithHistory, User, ContributorProfile, LoadingStatus as LoadingStatusType } from './interfaces'
import { LoadingStatus, PublishStage } from './interfaces'
import { fetchQuestion } from './data-loaders/fetch-question'
import { fetchQuestions } from './data-loaders/fetch-questions'


/// The main store for the application
export const useMainStore = defineStore('main', () => {

  const route = useRoute();

  /** The last loaded question in HTML format */
  const question = ref<Question | undefined>()

  /** The last loaded question in Markdown format 
   * TODO: replace this with a flag inside Question to indicate the format */
  const questionMD = ref<Question | undefined>()

  /** The last loaded list of questions */
  const questionsWithHistory = ref<QuestionWithHistory[] | undefined>()

  /// All topics selected by the user
  const selectedTopics = ref<string[]>([])

  /// Email from the token from the ID provider, e.g. Google or Github
  const email = ref<string | undefined>()

  /// Raw token from the ID provider, e.g. Google or Github
  const token = ref<string | undefined>()

  /// DDB profile data for the logged in user
  const user = ref<User | undefined>()

  /// Any data in the contributor profile should be ignored if this value is true
  const anonymousContributor = ref<boolean | undefined>()

  /** Changes as per the state of loadQuestion().
   * Check if the question is present.
   */
  const questionStatus = ref<LoadingStatusType | undefined>()

  /** Set by components that load question lists.
 */
  const questionListStatus = ref<LoadingStatusType | undefined>()

  /** Toggles the feedback form on and off */
  const feedbackStatus = ref<LoadingStatusType | undefined>();

  /** Toggles the refresher block on and off */
  const refresherLinksToggleFlag = ref<boolean>(false);

  /** Is set to true when the user clicks on Show Random Question,
   then back to false on loading the view that includes the sample question */
  const showingRandomQuestion = ref<boolean>(false)

  /** Topic ID from the URL query parameter */
  const currentTopic = computed((): string | undefined => {
    console.log(`Detected topic change to ${route.query[URL_PARAM_TOPIC]}`);
    return route.query.topic ? <string>route.query[URL_PARAM_TOPIC] : undefined;
  });

  const reset = () => {
    question.value = undefined;
    questionMD.value = undefined;
    questionsWithHistory.value = undefined;
    selectedTopics.value = [];
    showingRandomQuestion.value = false;
    email.value = undefined;
    token.value = undefined;
    user.value = undefined;
    anonymousContributor.value = undefined;
    questionStatus.value = undefined;
  }

  /// Reset the question fields to a blank state.
  /// This is a hack and there should be a more elegant solution.
  const resetQuestionToBlank = () => {

    // get the contributor details from the local storage
    const contribLS = localStorage.getItem(CONTRIBUTOR_DETAILS_LS_KEY);
    const contribParsed = contribLS ? JSON.parse(contribLS) : <ContributorProfile>{ name: "" };

    question.value = {
      qid: "",
      topic: "",
      question: "",
      answers: [],
      correct: 0,
      contributor: contribParsed,
      title: "",
      stage: PublishStage.Draft,
    };

    questionStatus.value = LoadingStatus.Loaded;
  }

  /** Loads a question from the DB or reuses the existing one if the topic and qid match.
   * Returns a random question for the topic if qid is not provided.
   * Updates the questionStatus value during the loading process.
   * Clear the question value to guarantee the latest version is loaded.
   */
  const loadQuestion = async (paramTopic: string, paramQid?: string) => {

    // hide the refresher links and feedback on reload
    refresherLinksToggleFlag.value = false;
    feedbackStatus.value = undefined;

    // check if the currently loaded question can be reused
    if (question.value && question.value.topic === paramTopic && question.value.qid === paramQid) {
      questionStatus.value = LoadingStatus.Loaded;
      console.log("Reusing the current question.");
      return;
    }

    // make sure nothing is showing if the component is reused
    questionStatus.value = LoadingStatus.Loading;
    question.value = undefined;  // clear the value so that other components know it's being reloaded

    if (!paramTopic) {
      console.error("No topic provided - using any.");
      paramTopic = randomTopicId();
    }

    const fetchedQuestion = await fetchQuestion(paramTopic, paramQid, token.value);

    if (fetchedQuestion) {
      // success
      question.value = fetchedQuestion;
      questionStatus.value = LoadingStatus.Loaded;
    }
    else if (fetchQuestion === null) {
      // no question found
      questionStatus.value = LoadingStatus.NoData;
    }
    else {
      // error
      questionStatus.value = LoadingStatus.Error;
    }
  };

  /** Loads the list of questions for the given topic, if not already loaded.
   * Finds the position of the currently loaded question in the list and loads the next one.
   * It loads the first question when the end of the list is reached.
   */
  const loadNextQuestion = async (topic: string) => {
    console.log("Load next question for topic: ", topic);


    // load the list of questions if none exists, or exists for a different topic
    if ((!questionsWithHistory.value || questionsWithHistory.value.length === 0) || questionsWithHistory.value[0].question.topic !== topic) {
      // load the list of questions for the given topic
      questionsWithHistory.value = await fetchQuestions(topic)
    }

    // TODO: report the error to the user
    if (!questionsWithHistory.value || questionsWithHistory.value.length === 0) {
      console.error("No questions found for the topic: ", topic);
      return;
    }

    // find the index of the current question by ID and pick the next one from the list or start from the beginning
    const currentQuestionIndex = (question.value ? questionsWithHistory.value.findIndex(q => q.question.qid === question.value?.qid) : -1) + 1;
    const nextQuestionIndex = currentQuestionIndex < questionsWithHistory.value.length ? currentQuestionIndex : 0;
    console.log(`currentQuestionIndex: ${currentQuestionIndex}, nextQuestionIndex: ${nextQuestionIndex}`);

    // load the next question
    loadQuestion(topic, questionsWithHistory.value[nextQuestionIndex].question.qid);
  };

  return {
    question,
    questionMD,
    questionsWithHistory,
    selectedTopics,
    currentTopic,
    showingRandomQuestion,
    email,
    token,
    user,
    anonymousContributor,
    questionStatus,
    questionListStatus,
    feedbackStatus,
    refresherLinksToggleFlag,
    reset,
    resetQuestionToBlank,
    loadQuestion,
    loadNextQuestion,
  }
})