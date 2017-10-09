#!/bin/sh
for i in `seq 1000`
do
echo hoge${i} | netcat localhost 3333
done
