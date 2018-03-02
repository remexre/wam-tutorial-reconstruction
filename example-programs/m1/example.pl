p(f(X), h(Y, f(a)), Y).

foo(f(X), g(X)).
foo(f(f), g(g)).
foo(X, X).

bar(_, _).

true.
anything(X).
'='(X, X).

% ?- p(W, h(W, Z), f(Z)).
% ?- foo(f(f), g(g)).
% ?- foo(f(f), g(f)).
% ?- foo(f(f), f(f)).
% ?- true.
% ?- anything(1).
% ?- anything(f(a)).
% ?- anything(Thing).
% ?- '='(foo, foo).
% ?- '='(Foo, Bar).
% ?- '='(a(b, C), a(B, c)).
