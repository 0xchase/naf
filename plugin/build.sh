#cargo rustc -- --emit=obj
#g++ -c -std=gnu++11 -O2 -Wall -W -fPIC -pipe -I /home/oem/software/binaryninja-api/ -c -o temp.o c-src/interface.cpp
#g++ -shared temp.o target/debug/deps/plugin.o libbinaryninjaapi.a -L /home/oem/software/binary-ninja -lbinaryninjacore -o breakpoint.so

# Build C++ to temp.o
g++ -c -std=gnu++11 -O2 -Wall -W -fPIC -pipe -I /home/oem/software/binaryninja-api/ -c -o temp.o c-src/interface.cpp

# Link all the files
g++ -shared temp.o libbinaryninjaapi.a -L /home/oem/github/ninja-analysis-framework/plugin/target/debug/ -lplugin -L /home/oem/software/binary-ninja -lbinaryninjacore -o breakpoint.so

# Copy library
cp breakpoint.so ~/software/binary-ninja/plugins/breakpoint.so
