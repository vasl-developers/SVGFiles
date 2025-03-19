#!/bin/bash

export BASH_ARGS=
export DESTINATION="./images"
export NATIONALITY="allied american axis british chinese communist finnish french german italian japanese russian swedish un"

while getopts dD:N:q option
do
	case "${option}" in
		d)
			BASH_ARGS="${BASH_ARGS} -d"
			;;
		D)
			DESTINATION=${OPTARG}
			;;
		N)
			NATIONALITY="${OPTARG}"
			;;			
		q)
			BASH_ARGS="${BASH_ARGS} -q"
			;;
	esac
done

BASH_ARGS="-D ${DESTINATION} ${BASH_ARGS}"
RUST_ARGS="--destination ${DESTINATION} ${RUST_ARGS}"

bash run_nationality.sh ${BASH_ARGS} -N "${NATIONALITY}"

bash run_miscellaneous.sh ${BASH_ARGS}
