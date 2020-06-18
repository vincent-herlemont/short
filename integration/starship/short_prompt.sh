#!/bin/bash

function blastoff(){
    setup=$(short show -s)
    env=$(short show -e)
    if [ "$setup" != "" ]; then
      echo -n "[$setup:$env] "
    fi
}
starship_precmd_user_func="blastoff"