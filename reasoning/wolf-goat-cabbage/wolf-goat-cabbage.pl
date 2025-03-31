% Wolf, goat and cabbage puzzle
% Original code at https://csci305.github.io/lectures/slides/Lecture33.pdf

% a solution is a starting configuration and a list of moves that takes you to
% [e, e, e, e], where all the intermediate configurations are safe
solution([e, e, e, e], []).
solution(Config, [Move|Rest]) :-
    move(Config, Move, NextConfig),
    safe(NextConfig),
    solution(NextConfig, Rest).

% each move transforms one configuration to another
% a configuration is list showing which bank man, wolf, goat, cabbage are on
move([X, X, Goat, Cabbage], wolf, [Y, Y, Goat, Cabbage]) :-
    change(X, Y).
move([X, Wolf, X, Cabbage], goat, [Y, Wolf, Y, Cabbage]) :-
    change(X, Y).
move([X, Wolf, Goat, X], cabbage, [Y, Wolf, Goat, Y]) :-
    change(X, Y).
move([X, Wolf, Goat, C], nothing, [Y, Wolf, Goat, C]) :-
    change(X, Y).

change(e, w).
change(w, e).

% safe if at least one of the goat or the wolf is on the same side as the man
% and at least one of the goat or the cabbage is on the same side as the man
safe([Man, Wolf, Goat, Cabbage]) :-
    oneEq(Man, Goat, Wolf),
    oneEq(Man, Goat, Cabbage).

oneEq(X, X, _).
oneEq(X, _, X).
