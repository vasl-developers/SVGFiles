#!/bin/bash

export DESTINATION="./images"

if [ $# -eq 1 ]
then
	DESTINATION=$1
fi

date

for i in al am ax br ch fi cc nk sk un us fr ge hu it ja ru ss
do
	echo "Cleaning ${i} ordnance"
	rm -f ${DESTINATION}/${i}/gun/*.svg
	echo "Cleaning ${i} vehicles"
	rm -f ${DESTINATION}/${i}/veh/*.svg
done

date
