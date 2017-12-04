function word2key(word, a, i, n, result)
{
    n = split(word, a, "")
    asort(a)

    for (i = 1; i <= n; i++)
        result = result a[i]

    return result
}

BEGIN {
	valid=0
}
{
	duplicates=0
	for (i=1;i<=NF;i++) {
		a[word2key($i)]++
	}
	for (x in a)
	{
		if(a[x]>1)
			duplicates++
	}
	for (x in a)
		delete a[x]

	if(duplicates==0)
		valid++
}
END {
	print valid
}
