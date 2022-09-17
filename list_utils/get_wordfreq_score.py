from wordfreq import zipf_frequency as zf
import json

data = {'answers':[]}
with open("answers_from_nyt.json") as f:
    words = json.load(f)
    for w in words["answers"]:
        data["answers"].append({'word':w, 'zipf_freq':zf(w, 'en')})


with open("words.json", "w") as f:    
    f.write(json.dumps(data, indent=4, sort_keys=True))