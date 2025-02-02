#!/bin/bash

function katuk { 
	case "$(katuk_rs "$@")" in
		cd$'\n'*)
			eval "cd \"$(katuk_rs "$@" | tail -n +2)\""
			;;
		*)
			katuk_rs "$@" | tail -n +2
			;;
	esac
}
