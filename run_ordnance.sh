#!/bin/bash

export BASH_ARGS=
export RUST_ARGS=
export DESTINATION="./images"
export NATIONALITY="allied american axis british chinese communist finnish french german italian japanese russian swedish un"

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

BASH_ARGS="-D ${DESTINATION} ${BASH_ARGS}"
RUST_ARGS="--destination ${DESTINATION} ${RUST_ARGS}"

for n in ${NATIONALITY}
do
	bash run_copy_cached_files.sh ${BASH_ARGS} -N ${n} -C gun
	
	CSV_FILE=data/${n}_ordnance.csv
	if [ -f "${CSV_FILE}" ]
	then
		echo "Generating ${j} ordnance"
		cargo run --bin generate_ordnance_counters -- ${RUST_ARGS} < ${CSV_FILE}
	fi
done
