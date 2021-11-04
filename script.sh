#!/usr/local/bin/bash
N=0
while true
do
	echo -n "$N: "
	RES=$(rg --count --max-count 1 ,$N, stripped)
	echo -n $RES
	if [[ "$RES" -eq "0" ]]
	then
		echo "Not found! "
		exit
	fi
	echo
	N=$((N+1))
done
