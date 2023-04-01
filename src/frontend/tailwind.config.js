/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/**/*.rs"],
  theme: {
    extend: {
      colors: {
      'gradient_color_1': '#1e293b',
      'gradient_color_2': '#140527',
    },
  },
  },
  plugins: [ ]
}
