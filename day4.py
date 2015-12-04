import sys
import md5

word = sys.argv[1]
number = 0

while True:
    digester = md5.new(word)
    digester.update(str(number))

    if digester.hexdigest()[0:6] == "000000":
        print word, number
        break

    number = number + 1
