#!/bin/bash
# shell script for cargo clean all project
PathArray=()
PathFile=path
index=0
index_inc=1
maxindex=24
while(( $index <= $maxindex ))
do
	PathArray[$index]=`sed -n ''${index_inc}'p' $PathFile`
	let "index++"
	let "index_inc++"
done

#index=0
#index_inc=1
#while(( $index <= $maxindex ))
#do
#	echo ${PathArray[$index]}
#	let "index++"
#	let "index_inc++"
#done

index=0
index_inc=1
while(( $index <= $maxindex ))
do
	echo ${PathArray[$index]}
	cd ${PathArray[$index]}
	cargo clean
	cd ..
	let "index++"
	let "index_inc++"
done
