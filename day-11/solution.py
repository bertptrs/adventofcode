import string

letters = [c for c in string.ascii_lowercase]

password = "hxbxwxba"

forbidden = set([letters.index(c) for c in "iol"])
passwordCode = [letters.index(c) for c in password]

def convert(password):
    return [letters.index(c) for c in password]

def isOk(password):
    hasStreak = False
    prev = -2
    streak = 1
    same = 0
    sameChar = None
    for x in password:
        if x in forbidden:
            return False

        if x == prev + 1:
            streak += 1
            if streak >= 3:
                hasStreak = True
        else:
            streak = 1

        if x == prev and x is not sameChar:
            same += 1
            sameChar = x

        prev = x

    return hasStreak and same >= 2

def incrementForbidden(password):
    for idx, x in enumerate(password):
        if x in forbidden:
            password[idx] = (x + 1) % len(letters)
            for i in range(idx + 1, len(password)):
                password[i] = 0

            return

def increment(password):
    carry = True
    i = len(password)
    while carry and i > 0:
        i -= 1
        password[i] = (password[i] + 1) % len(letters)
        carry = password[i] == 0

    incrementForbidden(password)

for x in range(2):
    increment(passwordCode)
    while not isOk(passwordCode):
        increment(passwordCode)

    print("Next password is", ''.join([letters[c] for c in passwordCode]))
