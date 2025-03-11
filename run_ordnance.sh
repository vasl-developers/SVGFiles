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
	CSV_FILE=data/${j}_ordnance.csv
	if [ -f "${CSV_FILE}" ]
	then
		echo "Generating ${j} ordnance"
		cargo run --bin generate_ordnance_counters -- ${DESTINATION} ${DEBUG} ${QUIET} < ${CSV_FILE}
	fi
done

date
