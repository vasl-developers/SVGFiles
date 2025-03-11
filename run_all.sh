#!/bin/bash

export BASH_ARGS=
export RUST_ARGS=
export DESTINATION="./images"

while getopts dD:q option
do
	case "${option}" in
		d)
			BASH_ARGS="${BASH_ARGS} -d"
			RUST_ARGS="${RUST_ARGS} --debug"
			;;
		D)
			DESTINATION=${OPTARG}
			;;
		q)
			BASH_ARGS="${BASH_ARGS} -q"
			RUST_ARGS="${RUST_ARGS} --quiet"
			;;
	esac
done

BASH_ARGS="-D ${DESTINATION} ${BASH_ARGS}"
RUST_ARGS="--destination ${DESTINATION} ${RUST_ARGS}"
			
date

for n in allied american axis british chinese communist finnish french german italian japanese russian un
do
	bash run_nationality.sh ${BASH_ARGS} -N ${n}
done

if [ -d "cached/sh" ]; then
	cp -r cached/sh ${DESTINATION}/sh
fi

if [ -d "cached/ML" ]; then
	cp -r cached/ML ${DESTINATION}/ML
fi

if [ -d "cached/MS" ]; then
	cp -r cached/MS ${DESTINATION}/MS
fi

CSV_FILE=data/landing_craft_and_boats.csv
if [ -f "${CSV_FILE}" ]
then	
	echo "Generating landing craft and boats"
	cargo run --bin generate_landing_craft_counters -- ${RUST_ARGS} < ${CSV_FILE}
fi

CSV_FILE=data/common_vehicles.csv
if [ -f "${VEH_FILE}" ]
then
	echo "Generating shared vehicles"
	cargo run --bin generate_vehicle_counters -- ${RUST_ARGS} < ${CSV_FILE}
fi

CSV_FILE=data/aircraft.csv
if [ -f "${CSV_FILE}" ]
then
	echo "Generating aircraft"
	cargo run --bin generate_aircraft_counters -- ${RUST_ARGS} < ${CSV_FILE}
fi
	
date

CSV_FILE=data/miscellaneous.csv
if [ -f "${CSV_FILE}" ]
then
	echo "Generating miscellaneous counters"
	cargo run --bin generate_miscellaneous_counters -- ${RUST_ARGS} < ${CSV_FILE}
fi
	
date
