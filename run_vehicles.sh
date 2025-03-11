#!/bin/bash

export DEBUG=
export DESTINATION="--destination ./images"
export QUIET=

while getopts dD:q option
do
	case "${option}" in
		d) DEBUG="--debug";;
		D) DESTINATION="--destination ${OPTARG}";;
		q) QUIET="--quiet";;
	esac
done

date

for j in allied american axis british chinese communist finnish french german italian japanese russian un
do
	CSV_FILE=data/${j}_vehicles.csv
	if [ -f "${CSV_FILE}" ]
	then
		echo "Generating ${j} vehicles"
		cargo run --bin generate_vehicle_counters -- ${DESTINATION} ${DEBUG} ${QUIET} < ${CSV_FILE}
	fi
done

CSV_FILE=data/landing_craft_and_boats.csv
if [ -f "${CSV_FILE}" ]
then
	echo "Generating landing craft and boats"
	cargo run --bin generate_landing_craft_counters -- ${DESTINATION} ${DEBUG} ${QUIET} < ${CSV_FILE}
fi

CSV_FILE=data/common_vehicles.csv
if [ -f "${CSV_FILE}" ]
then
	echo "Generating shared vehicles"
	cargo run --bin generate_vehicle_counters -- ${DESTINATION} ${DEBUG} ${QUIET} < ${CSV_FILE}
fi

CSV_FILE=data/aircraft.csv
if [ -f "${CSV_FILE}" ]
then
	echo "Generating aircraft"
	cargo run --bin generate_aircraft_counters -- ${DESTINATION} ${DEBUG} ${QUIET} < ${CSV_FILE}
fi

date
