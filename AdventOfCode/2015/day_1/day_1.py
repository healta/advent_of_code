with open(r"E:\Programming\AdventOfCode\AdventOfCode\2015\day_1\input.txt", "r") as f:
    text = f.read()
    openings = text.count("(")
    closings = text.count(")")
    print(openings-closings)
    result = 0
    running_count = 0
    for i in text:
        if i == "(":
            result+=1
        else:
            result-=1
        running_count +=1
        if result == -1:
            print(running_count)
            break
        