#!/usr/bin/env bash

__ScriptVersion="v0.1"

#===  FUNCTION  ================================================================
#         NAME:  usage
#  DESCRIPTION:  Display usage information.
#===============================================================================
function usage() {
	echo "Download iRails datasets from https://gtfs.irail.be/logs for the specified year

Usage :  $0 [options] [year]

    Options:
    -o|output-dir Output directory to store the download files
    -h|help       Display this message
    -v|version    Display script version"

} # ----------  end of function usage  ----------

#-----------------------------------------------------------------------
#  Handle command line arguments
#-----------------------------------------------------------------------

while getopts ":hvo" opt; do
	case $opt in
	o | output-dir)
		OUTPUTDIR=$2
		shift $(($OPTIND - 1))
		;;

	h | help)
		usage
		exit 0
		;;

	v | version)
		echo "$0 -- Version $__ScriptVersion"
		exit 0
		;;

	*)
		echo -e "\n  Option does not exist : $OPTARG\n"
		usage
		exit 1
		;;

	esac # --- end of case ---
done
shift $(($OPTIND - 1))

YEAR="${1:-"2019"}"
OUTPUTDIR="${OUTPUTDIR:-"irails_$YEAR"}"

echo "iRail logs of year $YEAR, will be downloaded into dir: $OUTPUTDIR"
mkdir -p $OUTPUTDIR

# slightly malformed input data
input_start="$YEAR-1-1"
input_end=$(date -I -d "$input_start + 1 year")

# After this, startdate and enddate will be valid ISO 8601 dates,
# or the script will have aborted when it encountered unparseable data
# such as input_end=abcd
startdate=$(date -d "$input_start" +%Y%m%d) || exit -1
enddate=$(date -d "$input_end" +%Y%m%d) || exit -1

TMPFILE="$(mktemp -t --suffix=.txt irails_download_sh.XXXXXX)"
trap "rm -f '$TMPFILE'" 0            # EXIT
trap "rm -f '$TMPFILE'; exit 1" 2    # INT
trap "rm -f '$TMPFILE'; exit 1" 1 15 # HUP TERM

IRAILS_LINK="https://gtfs.irail.be/logs/irailapi-"
SUFFIX=".log.tar.gz"
d="$startdate"
: >$TMPFILE
while [ "$d" != "$enddate" ]; do
	echo $IRAILS_LINK$d$SUFFIX >>${TMPFILE}
	d=$(date -d "$d + 1 day" +%Y%m%d)
done

#curl -O --output-dir $OUTPUTDIR

wget -P $OUTPUTDIR -i $TMPFILE
