/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./packages/client/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {},
  },
  daisyui: {
    themes: ["lofi", "black"],
  },
  plugins: [require("daisyui")],
};