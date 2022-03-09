
use crate::prelude::*;

#[allow(dead_code)]
pub static TEST_ANSWER_LIST: [WordleWord; 5] = 
[
    WordleWord {
        word: "aback",
        zipf_dist: 3.57
    },
    WordleWord {
        word: "abase",
        zipf_dist: 2.6
    },
    WordleWord {
        word: "abate",
        zipf_dist: 3.23
    },
    WordleWord {
        word: "abbey",
        zipf_dist: 2.18
    },
    WordleWord {
        word: "abbot",
        zipf_dist: 4.14
    }
];

pub static ANSWER_LIST: [WordleWord; 7] = 
[
	WordleWord { word: "cigar", zipf_dist: 3.57},
	WordleWord { word: "rebut", zipf_dist: 2.6},
	WordleWord { word: "sissy", zipf_dist: 3.23},
	WordleWord { word: "humph", zipf_dist: 2.18},
	WordleWord { word: "awake", zipf_dist: 4.14},
	WordleWord { word: "blush", zipf_dist: 3.56},
	WordleWord { word: "focal", zipf_dist: 3.67},

];