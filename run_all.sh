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

for j in allied american axis british chinese communist finnish french german italian japanese russian un
do
	ORD_FILE=data/${j}_ordnance.csv
	if [ -f "${ORD_FILE}" ]
	then
		echo "Generating ${j} ordnance"
		cargo run --bin generate_ordnance_counters ${DESTINATION} < ${ORD_FILE}
	fi

	VEH_FILE=data/${j}_vehicles.csv
	if [ -f "${VEH_FILE}" ]
	then
		echo "Generating ${j} vehicles"
		cargo run --bin generate_vehicle_counters ${DESTINATION} < ${VEH_FILE}
	fi	
done

date
