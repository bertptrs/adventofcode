BEGIN {
	valid=0
}
{
	duplicates=0
	for (i=1;i<=NF;i++) {
		a[$i]++
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
