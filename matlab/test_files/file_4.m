function w = proj(v, u)
    if(u == [0, 0, 0])
        error("can not project onto the zero vector");
    end
    w = (dot(u, v))/(dot(u, u)) * u;
end

closest = proj([1, 2,3], [5, 12, -7]);
distance = norm(closest - [1, 2, 3]);

s = [5, -1, 1];
t = [4, -3, 0];
n = cross(s, t);
v = [5.5, 4, 7];
proj_v_n = proj(v, n);

u = [2, 1, 1];
v = [-1 2 3];
A = zeros(150, 3);
for i = 1:150
    t = 10 * (rand - 0.5);
    s = 10 * (rand - 0.5);
    A(i, :) = t * v + s * u;
end
drawvectors(A);