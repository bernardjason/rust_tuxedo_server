# Tuxedo server service written in C and Rust

Make sure you look at env.sh which sets the location where Tuxedo is installed.
```
export TUXDIR=/software/tuxedo/tuxedo12.2.2.0.0
```

and where you've checked out the code
```
export BASE=/src/weblogic/rust_tuxedo_server
```


C bindings generated with

```
bindgen  tux.h  -o src/bindings.rs -- -I $TUXDIR/include/
```

then all the warnings ignored.


To compile
```
source env.sh
make all
```

Then to start
```
source env.sh
tmloadcf -y config/UBB.ubb
tmboot -y
```

Then test
```
ud32 -i ud.in
ud32 -i rust.in
```
