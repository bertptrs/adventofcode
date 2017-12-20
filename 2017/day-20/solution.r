#!/usr/bin/env Rscript
data <- readLines("stdin")
cleaned <- gsub(" $", "", gsub("^[^0-9-]+", "", gsub("[^0-9-]+", " ", data)))

tc <- textConnection(cleaned)

input <- read.csv(tc, header=FALSE, sep=" ")

close(tc)

accs <- rowSums(abs(input[,7:9]))
speeds <- rowSums(abs(input[,4:6]))

perm <- order(accs, speeds)
print(perm[1] - 1)


