#!/bin/bash

export DESTINATION="./images"

if [ $# -eq 1 ]
then
	DESTINATION=$1
fi

date

for j in allied american axis british chinese communist finnish french german italian japanese russian un
do
	SMC_FILE=data/${j}_smc.csv
	if [ -f "${SMC_FILE}" ]
	then
		echo "Generating ${j} single man counters"
		cargo run --bin generate_singleman_counters ${DESTINATION} < ${SMC_FILE}
	fi
	
	MMC_FILE=data/${j}_mmc.csv
	if [ -f "${MMC_FILE}" ]
	then
		echo "Generating ${j} multi man counters"
		cargo run --bin generate_multiman_counters ${DESTINATION} < ${MMC_FILE}
	fi	
done

date
