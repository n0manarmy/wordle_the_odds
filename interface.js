import init, { get_words } from './pkg/wordle_odds.js';
console.log('imported');

async function run() {
    await init();

    const submit_button = document.getElementById('submit');
    
    submit_button.addEventListener('click', () => {
        var first = (document.getElementById('first_placed').value != "") ? document.getElementById('first_placed').value : '\0';
        var second = (document.getElementById('second_placed').value != "") ? document.getElementById('second_placed').value : '\0';
        var third = (document.getElementById('third_placed').value != "") ? document.getElementById('third_placed').value : '\0';
        var fourth = (document.getElementById('fourth_placed').value != "") ? document.getElementById('fourth_placed').value : '\0';
        var fifth = (document.getElementById('fifth_placed').value != "") ? document.getElementById('fifth_placed').value : '\0';
        var found = (document.getElementById('found_letters').value != "") ? document.getElementById('found_letters').value : '\0';        
        console.log('first: ' + first);
        console.log('second: ' + second);
        console.log('third: ' + third);
        console.log('fourth: ' + fourth);
        console.log('fifth: ' + fifth);
        console.log('found: ' + found);
        console.log('submit button clicked');
        var results = get_words(first, second, third, fourth, fifth, found);
        console.log('results ' + results);
        var results_value = document.getElementById('results_value')
        results_value.textContent = results

    });
}

run();
