complete -c hyprdim -s s -l strength -d 'A value from 0 (no dim) to 1 (maximum dim)' -r
complete -c hyprdim -s d -l duration -d 'How many milliseconds to wait' -r
complete -c hyprdim -s f -l fade -d 'Fade animation speed' -r
complete -c hyprdim -s b -l bezier -d 'Bezier curve used for the animation' -r
complete -c hyprdim -s p -l persist -d 'Prevent dim_inactive from being disabled by `hyprctl reload` etc'
complete -c hyprdim -s h -l help -d 'Print help'
