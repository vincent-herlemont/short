#!/bin/bash

function blastoff(){
    setup=$(sht show -s)
    env=$(sht show -e)
    if [ "$setup" != "" ]; then
      echo -n "[$setup:$env] "
    fi
}
starship_precmd_user_func="blastoff"