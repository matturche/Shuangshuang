/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
    // transform: {
    //   rs: (content) => content.replace(/(?:^|\s)class:/g, ' '),
    // },
  },
  presets: [],
  darkMode: 'class',
  theme: {
      },
}
