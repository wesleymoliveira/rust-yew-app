module.exports = {
  purge: {
    mode: "all",
    content: [
      "./src/**/*.{rs,html,css}",
      "./index.html",
      "./src/components/**/*.{rs,html,css}",
    ],
  },
  theme: {
    extend: {},
  },
  plugins: [],
};
