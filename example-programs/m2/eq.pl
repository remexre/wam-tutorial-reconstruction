'='(X, X).

eq2(X, Z) :-
	'='(X, Y),
	'='(Y, Z).

% ?- '='(a, a).
% ?- '='(f(a), f(a)).
% ?- '='(h(X, y), h(x, Y)).