# Bus Extension

This Firefox extension adds some trivial enhancements to the "Where's My Bus" page of https://tstg.mybusplanner.ca.

## Features

* Reloads the page every 1 minute
* Adds an approximate distance (in km) between the bus and the first stop
* (TODO) Add a visual alert when the bus is close (map border flashes)

## Building

Run the following command to build the extension:
```sh
./package.sh
```

If you want it packaged as an XPI archive, set the `BUS_EXTENSION_PACK_FF` environment variable:
```sh
BUS_EXTENSION_PACK_FF=1 ./package.sh
```
