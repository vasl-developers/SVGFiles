#!/bin/bash

export DESTINATION="./images"

if [ $# -eq 1 ]
then
	DESTINATION=$1
fi

date

bash clean_all.sh ${DESTINATION}

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

VEH_FILE=data/landing_craft_and_boats.csv
if [ -f "${VEH_FILE}" ]
then
	echo "Generating landing craft and boats"
	cargo run --bin generate_landing_craft_counters ${DESTINATION} < ${VEH_FILE}
fi
VEH_FILE=data/shared_vehicles.csv
if [ -f "${VEH_FILE}" ]
then
	echo "Generating shared vehicles"
	cargo run --bin generate_vehicle_counters ${DESTINATION} < ${VEH_FILE}
fi
VEH_FILE=data/aircraft.csv
if [ -f "${VEH_FILE}" ]
then
	echo "Generating aircraft"
	cargo run --bin generate_aircraft_counters ${DESTINATION} < ${VEH_FILE}
fi
date
