sources=`find . -name "src" -type d`
curDir=`pwd`

for s in $sources
do
	base=`dirname $s`
	echo $base
	cd $base
	cargo clean
	cd $curDir
done
