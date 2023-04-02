/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/**/*.rs"],
  theme: {
    extend: {
      padding: {
        "1/2": "50%",
        full: "100%"
      },
      colors: {
        'gradient_color_1': '#1e293b',
        'gradient_color_2': '#140527'
      }
    },
  },
  plugins: [ ]
}
