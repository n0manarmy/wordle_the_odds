from wordfreq import zipf_frequency as zf
import json

file = open("../from_nyt.json")
words = json.load(file)

data = {'answers':[]}

for w in words["answers"]:
    data["answers"].append({'word':w, 'zipf_freq':zf(w, 'en')})
    # print(zf(w, 'en'))

file.close()

with open("../words.json", "w") as f:    
    f.write(json.dumps(data))
