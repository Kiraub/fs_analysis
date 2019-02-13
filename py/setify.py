str_set = set()
try:
    while True:
        istr = input().strip()
        if istr not in ["\n", ""]:
            str_set.add(istr)
except EOFError:
        for string in str_set:
            print(string)
