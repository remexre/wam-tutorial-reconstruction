p(f(X), h(Y, f(a)), Y).

foo(f(X), g(X)).
bar(f(f), g(g)).
baz(X, X).
quux(_, _).

true.
anything(X).
'='(X, X).

% ?- p(W, h(W, Z), f(Z)).
% ?- foo(f(f), g(g)).
% ?- bar(f(f), g(f)).
% ?- baz(f(f), f(f)).
% ?- quux(f(f), f(f)).
% ?- true.
% ?- anything(1).
% ?- anything(f(a)).
% ?- anything(Thing).
% ?- '='(foo, foo).
% ?- '='(Foo, Bar).
% ?- '='(a(b, C), a(B, c)).
