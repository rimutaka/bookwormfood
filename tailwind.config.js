const defaultTheme = require('tailwindcss/defaultTheme');

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{js,jsx,ts,tsx}"
  ],
  theme: {
    extend: {
      fontFamily: {
        display: ['"Amatic SC"', ...defaultTheme.fontFamily.sans],
        sans: ['"News Cycle"', ...defaultTheme.fontFamily.sans]
      }
    },
    plugins: [],
  }
}