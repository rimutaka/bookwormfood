import { createRouter, createWebHistory } from 'vue-router'
import { authGuard } from '@auth0/auth0-vue';
import { findTopicById, URL_PARAM_TOPIC } from '@/constants';

import HomeView from '../views/HomeView.vue'
import QuestionFormView from '@/views/QuestionFormView.vue'
import QuestionView from '@/views/QuestionView.vue'
import AboutView from '../views/AboutView.vue'
import SubscriptionView from '../views/SubscriptionView.vue'
import ContributeView from '@/views/ContributeView.vue';
import QuestionPreview from '@/views/QuestionPreview .vue';
import SponsorshipView from '@/views/SponsorshipView.vue';
import ThankYouView from '@/views/ThankYouView.vue';
import LoginView from '@/views/LoginView.vue';
import QuestionListView from '@/views/QuestionListView.vue';
import QuestionReviewView from '@/views/QuestionReviewView.vue';

// Extend RouteMeta to enforce title property stored in meta for every page
import 'vue-router'
declare module 'vue-router' {
  interface RouteMeta {
    /** This is set as HTML title by router.afterEach in the router module */
    title: string;
  }
}

/// A list of page names used elsewhere in the app
export const PageIDs = {
  HOME: 'home',
  CONTRIBUTE: 'contribute',
  ADD: 'add',
  QUESTION: 'question',
  QUESTIONS: 'questions',
  REVIEW: 'review',
  ABOUT: 'about',
  SUBSCRIPTION: 'subscription',
  PREVIEW: 'preview',
  SPONSORSHIP: 'gift-a-question',
  THANKYOU: 'thankyou',
  LOGIN: 'login',
}

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  scrollBehavior(to, from) {
    if (to.name == from.name) {
      return false;  // don't scroll if the user is just changing the query
    }
    else {
      return { top: 0 }; // always scroll to top if the page name is different
    }
  },
  routes: [
    {
      path: '/',
      name: PageIDs.HOME,
      component: HomeView,
    },
    {
      path: '/' + PageIDs.CONTRIBUTE,
      name: PageIDs.CONTRIBUTE,
      component: ContributeView,
      meta: { title: 'Contribute' }
    },
    {
      path: '/' + PageIDs.ADD,
      name: PageIDs.ADD,
      component: QuestionFormView,
      beforeEnter: authGuard,
      meta: { title: 'Edit question' }
    },
    {
      path: '/' + PageIDs.QUESTION,
      name: PageIDs.QUESTION,
      component: QuestionView,
      meta: { title: 'Question' }
    },
    {
      path: '/' + PageIDs.QUESTIONS,
      name: PageIDs.QUESTIONS,
      component: QuestionListView,
      meta: { title: 'Topics' }
    },
    {
      path: '/' + PageIDs.ABOUT,
      name: PageIDs.ABOUT,
      component: AboutView,
      meta: { title: 'About' }
    },
    {
      path: '/' + PageIDs.SUBSCRIPTION,
      name: PageIDs.SUBSCRIPTION,
      component: SubscriptionView,
      beforeEnter: authGuard,
      meta: { title: 'Subscription' }
    },
    {
      path: '/' + PageIDs.PREVIEW,
      name: PageIDs.PREVIEW,
      component: QuestionPreview,
      meta: { title: 'Preview' }
    },
    {
      path: '/' + PageIDs.SPONSORSHIP,
      name: PageIDs.SPONSORSHIP,
      component: SponsorshipView,
      meta: { title: 'Gift a question' }
    },
    {
      path: '/' + PageIDs.THANKYOU,
      name: PageIDs.THANKYOU,
      component: ThankYouView,
      meta: { title: 'Thank you!' }
    },
    {
      path: '/' + PageIDs.LOGIN,
      name: PageIDs.LOGIN,
      component: LoginView,
      meta: { title: 'Login' }
    },
    {
      path: '/' + PageIDs.REVIEW,
      name: PageIDs.REVIEW,
      component: QuestionReviewView,
      beforeEnter: authGuard,
      meta: { title: 'Question review' }
    },
  ]
})

router.afterEach((to, from) => {
  const topic = findTopicById(to.query[URL_PARAM_TOPIC]?.toString());
  const metaTitle = <string>to.meta?.title;

  // question pages have the topic name added at the front of the title
  if (to.name === PageIDs.QUESTION && topic) {
    // console.log(`Page title: ${window.document.title}`);
  }
  else if (to.name === PageIDs.QUESTIONS && topic) {
    window.document.title = topic ? `${topic} questions` : metaTitle;
  }
  else {
    // other pages use the hardcoded meta property
    const defaultTitle = 'Bite-sized learning';
    window.document.title = metaTitle ? `${metaTitle} | ${defaultTitle}` : defaultTitle;
  }
});

export default router
