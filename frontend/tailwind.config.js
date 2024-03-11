module.exports = {
  mode: 'jit',
  darkMode: 'media', // or 'media' or 'class'
  content: ['./src/**/*.rs', './index.html'],
  // content: ["./src/**/*.{js,jsx,ts,tsx}", "./public/index.html"],
  theme: {
    fontFamily: {
      display: ['Open Sans', 'sans-serif'],
      body: ['Open Sans', 'sans-serif'],
    },
    maxHeight: {
      '1/4': '25%',
      '1/2': '50%',
      '3/4': '75%',
    },
    extend: {
      screens: {
        mf: '990px',
      },
      keyframes: {
        'slide-in': {
          '0%': {
            '-webkit-transform': 'translateX(120%)',
            transform: 'translateX(120%)',
          },
          '100%': {
            '-webkit-transform': 'translateX(0%)',
            transform: 'translateX(0%)',
          },
        },
      },
      animation: {
        'slide-in': 'slide-in 0.5s ease-out',
      },
    },
  },
  plugins: [require('@tailwindcss/forms')],
  variants: {
    extend: {},
  },
}

//https://tailwindcss.com/docs/vite/

//This Project will use Tailwindcss .
//Installing tailwindcss for vite (process is the same for create-react-app)
// -> npm install -D tailwindcss postcss autoprefixer
// -> npx tailwindcss init -p
// -> copy the template paths provided by tailwind and paste it in tailwind.config.js ( which I already did as you can see)
// -> Inside src folder copy and paste tailwind directives inside index.css by deleting all the inital(code that appears after initializing vite app) written code
