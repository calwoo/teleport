#! /bin/bash

# NOTE: put this function in your .bashrc

function tp() {
    OUTPUT=$( tp-exe $@ )

    if [ $? -eq 4 ]
    then
        cd $OUTPUT
    else
        echo $OUTPUT
    fi
}