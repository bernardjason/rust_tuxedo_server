export BASE=/src/weblogic/rust_tuxedo_server
export TUXDIR=/software/tuxedo/tuxedo12.2.2.0.0

mkdir -p $BASE/bin $BASE/logs


export APPDIR=$BASE/bin
export TUXCONFIG=$BASE/config/UBB
export ULOGPFX=$BASE/logs/ULOG
export PATH=$PATH:$TUXDIR/bin
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$TUXDIR/lib:$BASE/tuxedorust/target/release:$APPDIR/bin:
export FLDTBLDIR32=$TUXDIR/udataobj:$BASE/data
export FIELDTBLS32=tpadm,Usysfl32,flds
export LANG=C
export PMID=jasonb

cat $BASE/config/UBB.template | sed \
	-e "s!APPDIR=.*!APPDIR=\"$APPDIR\"!g" \
	-e "s!TUXCONFIG=.*!TUXCONFIG=\"$TUXCONFIG\"!g" \
	-e "s!TUXDIR=.*!TUXDIR=\"$TUXDIR\"!g" \
	-e "s!ULOGPFX=.*!ULOGPFX=\"$ULOGPFX\"!g" > $BASE/config/UBB.ubb

