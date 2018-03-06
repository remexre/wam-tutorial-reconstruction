evalBool(false, false).
evalBool(true, true).

evalBool(and(X, Y), true) :-
	evalBool(X, true),
	evalBool(Y, true).
evalBool(and(X, _), false) :- evalBool(X, false).
evalBool(and(_, Y), false) :- evalBool(Y, false).

evalBool(or(X, _), true) :- evalBool(X, true).
evalBool(or(_, Y), true) :- evalBool(Y, true).
evalBool(or(X, Y), false) :-
	evalBool(X, false),
	evalBool(Y, false).

evalBool(not(X), true) :- evalBool(X, false).
evalBool(not(X), false) :- evalBool(X, true).
