def main(path: str, cols: list, loc: int):
    with open(path, "r+", encoding="utf-8") as f:
        lines = f.read().splitlines()
    txt = []
    file = path.replace(".txt", ".csv")
    reps = ["{", "}", "=", "\""]
    for line in lines:
        for r in reps + cols:
            line = line.replace(r, "")
        txt.append(line.split(";"))
    with open(file, "w+", encoding="utf-8") as f:
        f.write(",".join(cols)+"\n")
        for line in txt:
            f.write(",".join(line[:loc]))
            f.write("\n")

main("data\\eki.txt", ["kanji", "kana", "romaji", "shozoku"], 4)
main("data\\ekikan.txt", ["kiten", "shuten", "keiyu", "kyori", "jikan"], 5)