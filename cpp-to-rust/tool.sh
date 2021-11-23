#!/bin/bash

check(){
	make
	valgrind --tool=memcheck --leak-check=full --show-leak-kinds=all ./run
}
help(){
	echo "sh tool.sh check -- check leak"
}
case $1 in
	check) check;;
	*) help;;
esac