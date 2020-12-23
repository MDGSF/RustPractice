g++ -c ReadBuffer.cpp
ar crv libReadBuffer.a ReadBuffer.o
g++ -shared -fPIC -o libReadBuffer.so ReadBuffer.o
