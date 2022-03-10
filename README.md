# About
This is a Rust/WASM and JavaScript applet that is intended to provide the odds of guessing the correct wordle puzzle answer based on the correct and found letters.

# Directions
To setup the program to run, I pull the word lists from the wordle site. The answer list gets placed into a json file in the list_utils directory called from_nyt.json. The file is formatted like below:

```
{
    "answers":[
        "word",
        "word",
        "word"
    ]
}
```

Next the python program in list_utils, get_wordfreq_score.py, is run against the file and provides a word frequency score to each word. This gets output to words.json. The build.rs file will automatically parse this to words.rs.

# Requirements
I use this library for the word frequency https://github.com/rspeer/wordfreq