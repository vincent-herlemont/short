#!/bin/bash

function blastoff(){
    echo $(sht show -f)
}
starship_precmd_user_func=blastoff