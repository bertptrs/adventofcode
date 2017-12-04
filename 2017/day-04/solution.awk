function word2key(word, a, i, n, result)
{
    n = split(word, a, "")
    asort(a)

    for (i = 1; i <= n; i++)
        result = result a[i]

    return result
}

BEGIN {
	valid1=0
	valid2=0
}
{
	duplicates=0
	anagrams=0
	for (i=1;i<=NF;i++) {
		a[word2key($i)]++
		b[$i]++
	}
	for (x in a)
	{
		if (a[x] > 1)
			anagrams++
		delete a[x]
	}
	for (x in b) {
		if (b[x] > 1)
			duplicates++

		delete b[x]
	}

	if (duplicates == 0)
		valid1++

	if (anagrams == 0)
		valid2++
}
END {
	print valid1
	print valid2
}

