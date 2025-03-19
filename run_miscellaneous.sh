#!/bin/bash

export RUST_ARGS=
export DESTINATION="./images"

while getopts dD:q option
do
	case "${option}" in
		d)
			RUST_ARGS="${RUST_ARGS} --debug"
			;;
		D)
			DESTINATION="${OPTARG}"
			;;
		q)
			RUST_ARGS="${RUST_ARGS} --quiet"
			;;
	esac
done

RUST_ARGS="--destination ${DESTINATION} ${RUST_ARGS}"

if [ -d "cached/sh" ]; then
	if [ -d "cached/sh/svg" ] ; then
		cp -r cached/sh/svg/* ${DESTINATION}/sh/svg
	fi	
fi

if [ -d "cached/ML" ]; then
	if [ -d "cached/ML/svg" ] ; then
		cp -r cached/ML/svg/* ${DESTINATION}/ML/svg
	fi	
fi

if [ -d "cached/MS" ]; then
	if [ -d "cached/MS/svg" ] ; then
		cp -r cached/MS/svg/* ${DESTINATION}/MS/svg
	fi	
fi

CSV_FILE=data/landing_craft_and_boats.csv
if [ -f "${CSV_FILE}" ]
then
	echo "Generating landing craft and boats"
	cargo run --bin generate_landing_craft_counters -- ${RUST_ARGS} < ${CSV_FILE}
fi

CSV_FILE=data/common_vehicles.csv
if [ -f "${CSV_FILE}" ]
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

CSV_FILE=data/miscellaneous.csv
if [ -f "${CSV_FILE}" ]
then
	echo "Generating miscellaneous counters"
	cargo run --bin generate_miscellaneous_counters -- ${RUST_ARGS} < ${CSV_FILE}
fi
