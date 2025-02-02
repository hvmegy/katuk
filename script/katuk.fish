#!/usr/bin/env fish
function katuk
	set signal (katuk_rs $argv)
	set type $signal[1]
	set output $signal[2..-1]

	switch $type
		case 'cd'
			cd $output
			echo "Changed directory to $output"
		case '*'
			for line in $output 
				echo $line
			end
	end
end

