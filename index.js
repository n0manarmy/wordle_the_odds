// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
// const rust = import('./pkg/wordle_odds.js');
const submit_button = document.getElementById('submit')

submit_button.addEventListener('click', () => {
  console.log('submit button clicked')
  // rust.then(m => m.get_words('World!')).catch(console.error);
});