import init, { get_words } from './pkg/wordle_odds.js';
console.log('imported');

async function run() {
    await init();

    const q_button = document.getElementById('q_button');
    const w_button = document.getElementById('w_button');
    const e_button = document.getElementById('e_button');
    const r_button = document.getElementById('r_button');
    const t_button = document.getElementById('t_button');
    const y_button = document.getElementById('y_button');
    const u_button = document.getElementById('u_button');
    const i_button = document.getElementById('i_button');
    const o_button = document.getElementById('o_button');
    const p_button = document.getElementById('p_button');
    const a_button = document.getElementById('a_button');
    const s_button = document.getElementById('s_button');
    const d_button = document.getElementById('d_button');
    const f_button = document.getElementById('f_button');
    const g_button = document.getElementById('g_button');
    const h_button = document.getElementById('h_button');
    const j_button = document.getElementById('j_button');
    const k_button = document.getElementById('k_button');
    const l_button = document.getElementById('l_button');
    const z_button = document.getElementById('z_button');
    const x_button = document.getElementById('x_button');
    const c_button = document.getElementById('c_button');
    const v_button = document.getElementById('v_button');
    const b_button = document.getElementById('b_button');
    const n_button = document.getElementById('n_button');
    const m_button = document.getElementById('m_button');

    const buttons = [
        q_button,
        w_button,
        e_button,
        r_button,
        t_button,
        y_button,
        u_button,
        i_button,
        o_button,
        p_button,
        a_button,
        s_button,
        d_button,
        f_button,
        g_button,
        h_button,
        j_button,
        k_button,
        l_button,
        z_button,
        x_button,
        c_button,
        v_button,
        b_button,
        n_button,
        m_button];

    buttons.forEach(b => b.addEventListener('click', incorrect_click));

    const submit_button = document.getElementById('submit_button');

    submit_button.addEventListener('click', () => {
        var first = (document.getElementById('first_placed').value != "") ? document.getElementById('first_placed').value : '\0';
        var second = (document.getElementById('second_placed').value != "") ? document.getElementById('second_placed').value : '\0';
        var third = (document.getElementById('third_placed').value != "") ? document.getElementById('third_placed').value : '\0';
        var fourth = (document.getElementById('fourth_placed').value != "") ? document.getElementById('fourth_placed').value : '\0';
        var fifth = (document.getElementById('fifth_placed').value != "") ? document.getElementById('fifth_placed').value : '\0';
        var found = (document.getElementById('found_letters').value != "") ? document.getElementById('found_letters').value : '\0';
        // console.log('first: ' + first);
        // console.log('second: ' + second);
        // console.log('third: ' + third);
        // console.log('fourth: ' + fourth);
        // console.log('fifth: ' + fifth);
        // console.log('found: ' + found);
        // console.log('submit button clicked');
        var incorrect = buttons.filter(button => button.classList.contains("active"))
            .map(button => {
                return button.innerHTML;
            });
        // console.log(incorrect)

        var results = get_words(first, second, third, fourth, fifth, found, incorrect);
        // console.log('results ' + results);

        var results_value = document.getElementById('results_value')
        results_value.textContent = results

    });
}

async function incorrect_click(event) {
    const button = event.target
    console.log(button.innerHTML + ' button clicked');
    if (button.classList.contains("active")) {
        button.classList.remove("active");
    } else {
        button.classList.add("active")
    }
}


run();
