#!/bin/bash

export BASH_ARGS=
export RUST_ARGS=
export DESTINATION="./images"
export NATIONALITY=

while getopts dD:N:q option
do
	case "${option}" in
		d)
			BASH_ARGS="${BASH_ARGS} -d"
			RUST_ARGS="${RUST_ARGS} --debug"
			;;
		D)
			DESTINATION="${OPTARG}"
			;;
		N)
			NATIONALITY="${OPTARG}"
			;;
		q)
			BASH_ARGS="${BASH_ARGS} -q"
			RUST_ARGS="${RUST_ARGS} --quiet"
			;;
	esac
done

BASH_ARGS="-D ${DESTINATION} -N ${NATIONALITY} ${BASH_ARGS} -C all"
RUST_ARGS="--destination ${DESTINATION} ${RUST_ARGS}"

for n in ${NATIONALITY}
do
	bash run_copy_cached_files.sh ${BASH_ARGS}

	CSV_FILE=data/${n}_ordnance.csv
	if [ -f "${CSV_FILE}" ]
	then
		echo "Generating ${n} ordnance"
		cargo run --bin generate_ordnance_counters -- ${RUST_ARGS} < ${CSV_FILE}
	fi

	CSV_FILE=data/${n}_vehicles.csv
	if [ -f "${CSV_FILE}" ]
	then
		echo "Generating ${n} vehicles"
		cargo run --bin generate_vehicle_counters -- ${RUST_ARGS} < ${CSV_FILE}
	fi

	CSV_FILE=data/${n}_smc.csv
	if [ -f "${CSV_FILE}" ]
	then
		echo "Generating ${n} single man counters"
		cargo run --bin generate_singleman_counters -- ${RUST_ARGS} < ${CSV_FILE}
	fi

	CSV_FILE=data/${n}_mmc.csv
	if [ -f "${CSV_FILE}" ]
	then
		echo "Generating ${n} multi-man counters"
		cargo run --bin generate_multiman_counters -- ${RUST_ARGS} < ${CSV_FILE}
	fi
	
	CSV_FILE=data/${n}_sw.csv
	if [ -f "${CSV_FILE}" ]
	then
		echo "Generating ${n} support weapon counters"
		cargo run --bin generate_sw_counters -- ${RUST_ARGS} < ${CSV_FILE}
	fi	
done
