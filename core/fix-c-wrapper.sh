#!/bin/bash

DIRECTORY="vendor/verovio/tools"

cd $DIRECTORY

sed -i '1s/^/#include "stdbool.h";\n/' c_wrapper.h
sed -i '1s/^/typedef void Toolkit;\n/' c_wrapper.h
