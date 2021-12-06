#! /bin/bash

function tp() {
    OUTPUT=$( tp-exe $@ )

    if [ $? -eq 4 ]
    then
        cd $OUTPUT
    else
        echo $OUTPUT
    fi
}