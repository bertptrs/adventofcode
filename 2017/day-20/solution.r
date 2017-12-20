#!/usr/bin/env Rscript
data <- readLines("stdin")
cleaned <- gsub(" $", "", gsub("^[^0-9-]+", "", gsub("[^0-9-]+", " ", data)))

tc <- textConnection(cleaned)

input <- read.csv(tc, header=FALSE, sep=" ")

close(tc)

perm <- order(rowSums(abs(input[,7:9])), rowSums(abs(input[,4:6])))
print(perm[1] - 1)

for (i in 1:100) {
	# Update positions, speeds
	input[,4:6] <- input[,4:6] + input[,7:9]
	input[,1:3] <- input[,1:3] + input[,4:6]

	dups <- !(duplicated(input[,1:3]) | duplicated(input[,1:3], fromLast=TRUE))

	input <- input[dups,]
}

print(nrow(input))
