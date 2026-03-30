u = [1, 3.8, 5.2];
v = [-1.1, 2.3, -4.5];
q1a = norm(u)
q1bi = dot(u, v)
q1bii = "same"
q1c = dot(cross(u, v), u) == 0 && dot(cross(u, v), v) == 0
q1d = cross(u, u)
k = 3.1;
q1ei = dot((k * u), v)
q1eii = dot(u, v) * k
q1f = "error"
w = [1.5, 2.5, 3.5];
q1g = dot(u, cross(v, w))

