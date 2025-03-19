#!/bin/bash

export CATEGORY="all"
export DESTINATION="./images"
export NATIONALITY=
export QUIET_ARG=
export DIRS=
export QUIET="no"

while getopts C:D:N:q option
do
	case "${option}" in
		C)
			CATEGORY="${OPTARG}"
			;;
		D)
			DESTINATION="${OPTARG}"
			;;
		N)
			NATIONALITY="${OPTARG}"
			;;
		q)
			QUIET_ARG="quiet"
			QUIET="yes"
			;;
	esac
done

for n in ${NATIONALITY}
do
	case ${n} in
		"allied")
			DIRS=("al" "et");;
		"american")
			DIRS=("am" "us");;
		"axis")
			DIRS=("ax" "hu");;
		"british")
			DIRS=("br");;
		"chinese")
			DIRS=("ch");;
		"communist")
			DIRS=("cc" "nk");;
		"finnish")
			DIRS=("fi");;
		"french")
			DIRS=("ff" "fr" "vf");;
		"german")
			DIRS=("ge" "ss");;
		"italian")
			DIRS=("it" "er");;
		"japanese")
			DIRS=("ja");;
		"russian")
			DIRS=("ru" "pa");;
		"swedish")
			DIRS=("sv");;			
		"un")
			DIRS=("un" "sk");;
	esac

	for d in "${DIRS[@]}"; do	
		if [[ -d "cached/${d}/gun/svg" && ( "${CATEGORY}" = "all" || "${CATEGORY}" = "gun" ) ]]; then
			if [ "${QUIET}" = "no" ]
			then
				echo "Copying ${d} ordnance depiction files"
			fi
			
			cp -r cached/${d}/gun/svg ${DESTINATION}/${d}/gun
		fi
		
		if [[ -d "cached/${d}/svg" && ( "${CATEGORY}" = "all" || "${CATEGORY}" = "inf" ) ]]; then
			if [ "${QUIET}" = "no" ]
			then
				echo "Copying ${d} infantry unit depiction files"
			fi
			
			cp -r cached/${d}/svg ${DESTINATION}/${d}
		fi

		if [[ -d "cached/${d}/veh/svg" && ( "${CATEGORY}" = "all" || "${CATEGORY}" = "veh" ) ]]; then
			if [ "${QUIET}" = "no" ]
			then
				echo "Copying ${d} vehicle depiction files"
			fi
			
			cp -r cached/${d}/veh/svg ${DESTINATION}/${d}/veh
		fi				
	done
done
