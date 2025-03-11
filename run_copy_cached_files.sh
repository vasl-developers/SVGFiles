#!/bin/bash

export DESTINATION="./images"
export NATIONALITY=
export QUIET_ARG=
export DIRS=
export QUIET="no"

while getopts D:N:q option
do
	case "${option}" in
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

if [ "${QUIET}" = "no" ]
then
	date
fi

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
		"un")
			DIRS=("un" "sk");;
	esac

	for d in "${DIRS[@]}"; do	
		if [ -d "cached/${d}/gun/svg" ]; then
			if [ "${QUIET}" = "no" ]
			then
				echo "Copying ${d} ordnance depiction files"
			fi
			
			cp -r cached/${d}/gun/svg ${DESTINATION}/${d}/gun
		fi
		
		if [ -d "cached/${d}/svg" ]; then
			if [ "${QUIET}" = "no" ]
			then
				echo "Copying ${d} unit depiction files"
			fi
			
			cp -r cached/${d}/svg ${DESTINATION}/${d}
		fi

		if [ -d "cached/${d}/veh/svg" ]; then
			if [ "${QUIET}" = "no" ]
			then
				echo "Copying ${d} vehicle depiction files"
			fi
			
			cp -r cached/${d}/veh/svg ${DESTINATION}/${d}/veh
		fi				
	done
done
	
if [ "${QUIET}" = "no" ]
then
	date
fi
