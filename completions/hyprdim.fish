complete -c hyprdim -s s -l strength -d 'A value from 0 (no dim) to 1 (maximum dim)' -r
complete -c hyprdim -s d -l duration -d 'How many milliseconds to wait before removing dim' -r
complete -c hyprdim -s f -l fade -d 'Fade animation speed from 0 (instantaneous) to 255 (very slow)' -r
complete -c hyprdim -s b -l bezier -d 'Bezier curve used for the animation' -r
complete -c hyprdim -s S -l dialog-strength -d 'How much to dim same-class floating windows' -r
complete -c hyprdim -s p -l persist -d 'Prevent dim_inactive from being disabled by `hyprctl reload` etc'
complete -c hyprdim -s n -l no-dim-when-only -d 'Don\'t dim when switching to a workspace that only has one visible window'
complete -c hyprdim -s i -l ignore-entering-special -d 'Don\'t dim when opening a special workspace'
complete -c hyprdim -s I -l ignore-leaving-special -d 'Don\'t dim when closing a special workspace'
complete -c hyprdim -s D -l dialog-dim -d 'Dim windows if they\'re the same class and floating'
complete -c hyprdim -s v -l verbose -d 'Show information about what hyprdim is doing'
complete -c hyprdim -s h -l help -d 'Print help (see more with \'--help\')'
complete -c hyprdim -s V -l version -d 'Print version'
