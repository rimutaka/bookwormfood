<template>
  <div class="about">
    <p>This website is an open education project by <a href="https://github.com/rimutaka">Max @rimutaka</a>,
      a software engineer from Auckland, New Zealand.</p>
    <p>It was inspired by the success of Stack Overflow, FreeCodeCamp and other online learning communities.</p>
    <p>This project started in September 2024. It is a work-in-progress and <router-link :to="PageIDs.CONTRIBUTE">you can help make it better</router-link>.</p>

    <h3>Microlearning through questions</h3>
    <p>Questions are a great way to identify gaps in knowledge.</p>
    <p>Answers with detailed explanations are an easy way to fill those gaps.</p>
    <p>Learning through questions is not a substitute for other types of learning or practicing a skill,
      but doing it consistently adds up.</p>

    <h3>No scoring or leaderboards</h3>
    <p>Your results and performance stats are available to you only.</p>
    <p>Learning is not a race and the only winner is you.</p>

    <h3>How subscriptions work</h3>
    <p>You get one email per day with a random question about one of your selected topics.</p>
    <p>The next email is scheduled for the following day when you answer the previous question.</p>
    <p>Emails are automatically paused if you stop answering questions to minimize your backlog.</p>
    <p>You can change or remove your topics on <router-link :to="PageIDs.SUBSCRIPTION">subscription</router-link> page and every email has a one-click unsubscribe link.</p>

    <h3>Privacy</h3>
    <p>No data from this app is shared with any third party.</p>
    <ul class=" text-start">
      <li>Authentication: <a href="https://auth0.com">Auth0</a></li>
      <li>Infrastructure: AWS</li>
      <li>Source code: <a href="https://github.com/rimutaka/microlearning">https://github.com/rimutaka/microlearning</a></li>
    </ul>

    <h3>Feedback</h3>
    <p>Your feedback is very welcome. The best way to get in touch is to <a href="mailto:max@bitesized.info">email the maintainer</a>.</p>
    <p>Your help with making the site better will be much appreciated.</p>
    <p>If you run into bugs, press <em>Ctrl+Shift+I</em> and copy-paste the console output into the email.</p>
  </div>
</template>

<script setup lang="ts">

import { PageIDs } from '@/router';

</script>