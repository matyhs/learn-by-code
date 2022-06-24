const colors = require('tailwindcss/colors')

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        brand: {
          primary: 'rgb(var(--color-primary) / <alpha-value>)',
          secondary: 'rgb(var(--color-secondary) / <alpha-value>)',
          tertiary: 'rgb(var(--color-tertiary) / <alpha-value>)'
        },
      },
    },
  },
  plugins: [],
}
