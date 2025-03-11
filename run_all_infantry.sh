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
	CSV_FILE=data/${j}_smc.csv
	if [ -f "${CSV_FILE}" ]
	then
		echo "Generating ${j} single man counters"
		cargo run --bin generate_singleman_counters -- ${DESTINATION} ${DEBUG} ${QUIET} < ${CSV_FILE}
	fi
	
	CSV_FILE=data/${j}_mmc.csv
	if [ -f "${CSV_FILE}" ]
	then
		echo "Generating ${j} multi man counters"
		cargo run --bin generate_multiman_counters -- ${DESTINATION} ${DEBUG} ${QUIET} < ${CSV_FILE}
	fi
	
	CSV_FILE=data/${j}_sw.csv
	if [ -f "${CSV_FILE}" ]
	then
		echo "Generating ${j} support weapon counters"
		cargo run --bin generate_sw_counters -- ${DESTINATION} ${DEBUG} ${QUIET} < ${CSV_FILE}
	fi	
done

date
