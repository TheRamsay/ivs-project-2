/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/**/*.rs"],
  theme: {
    extend: {},
    colors: {
      'gradient_color_1': '#1E293B',
      'gradient_color_2': '#140527',
    },
  },
  plugins: [ ]
}
