import { createRouter, createWebHistory } from 'vue-router'
import { authGuard } from '@auth0/auth0-vue';
import { findTopicById, URL_PARAM_TOPIC } from '@/constants';

import WelcomeView from '../views/WelcomeView.vue'
import ScanView from '@/views/ScanView.vue'
import BookDetailsView from '@/views/BookDetailsView.vue'
import LoginView from '@/views/LoginView.vue';

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
  BOOK: 'book',
  SCAN: 'scan',
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
      component: WelcomeView,
    },
    {
      path: '/' + PageIDs.SCAN,
      name: PageIDs.SCAN,
      component: ScanView,
      meta: { title: 'Scan book barcode' }
    },
    // {
    //   path: '/' + PageIDs.ADD,
    //   name: PageIDs.ADD,
    //   component: QuestionFormView,
    //   beforeEnter: authGuard,
    //   meta: { title: 'Edit question' }
    // },
 
  ]
})

// router.afterEach((to, from) => {
//   const topic = findTopicById(to.query[URL_PARAM_TOPIC]?.toString());
//   const metaTitle = <string>to.meta?.title;

//   // question pages have the topic name added at the front of the title
//   if (to.name === PageIDs.QUESTION && topic) {
//     // console.log(`Page title: ${window.document.title}`);
//   }
//   else if (to.name === PageIDs.QUESTIONS && topic) {
//     window.document.title = topic ? `${topic} questions` : metaTitle;
//   }
//   else {
//     // other pages use the hardcoded meta property
//     const defaultTitle = 'Bite-sized learning';
//     window.document.title = metaTitle ? `${metaTitle} | ${defaultTitle}` : defaultTitle;
//   }
// });

export default router
