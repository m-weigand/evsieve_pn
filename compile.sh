#!/usr/bin/env sh

cd /root/

git clone --branch main https://github.com/m-weigand/evsieve_pn.git

cd evsieve_pn
# test -d build && rm -r build
# mkdir build
# cd build

# # cmake -D DEBUG_INPUT=1 -D DEBUG_INPUT_PRINT_ALL_MOTION_EVENTS=1 .. -DCPACK_GENERATOR="DEB" ..
# cmake .. -DCPACK_GENERATOR="DEB" ..
# make -j 2
# cmake --build . --target package
# tree
# pwd
# mv packages/*.deb /github/home/
