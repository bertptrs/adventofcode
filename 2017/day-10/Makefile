.PHONY: all

all: solution.jar
	java -jar $<

solution.jar: solution.kt
	kotlinc $< -include-runtime -d $@

