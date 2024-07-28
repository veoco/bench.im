/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
      "./index.html",
      "./src/**/*.{js,jsx}",
    ],
    plugins: [
      require('@tailwindcss/typography'),
      require('@tailwindcss/forms'),
    ],  
  }