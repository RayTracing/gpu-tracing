import matplotlib.pyplot as plt
import numpy as np
import math

# Fixing random state for reproducibility
np.random.seed(19680801)

def randrange(n, vmin, vmax):
    """
    Helper function to make an array of random numbers having shape (n, )
    with each number distributed Uniform(vmin, vmax).
    """
    return (vmax - vmin)*np.random.rand(n) + vmin

def sphere_plot(n):
    rands = np.random.rand(n, 2)
    xs = []
    ys = []
    zs = []
    pxs = []
    pys = []
    pzs = []
    for rnd in rands:
        z = 1 - 2 * rnd[0]
        r = math.sqrt(1 - z * z)
        phi = 2 * math.pi * rnd[1]
        x = r * math.cos(phi)
        y = r * math.sin(phi)

        xs.append(x)
        ys.append(y)
        zs.append(z)

        pz = z + 1
        l = 0.5 * math.sqrt(x*x + y*y + pz*pz)
        pxs.append(x / l)
        pys.append(y / l)
        pzs.append((pz / l) - 1)

        #pxs.append(x / l)
        #pys.append(y / l)
        #pzs.append(-pz / l - 1)

    return [xs, ys, zs, pxs, pys, pzs]


fig = plt.figure()
ax = fig.add_subplot(projection='3d')
#ax.set_adjustable('box')
#ax.axis('equal')

n = 1000
plot = sphere_plot(n)
ax.scatter(plot[0], plot[1], plot[2], marker='o', s=2, c="#FFC107", alpha=0.8)
ax.scatter(plot[3], plot[4], plot[5], marker='o', s=2, c="#1E88E5", alpha=0.5)

# For each set of style and range settings, plot n random points in the box
# defined by x in [23, 32], y in [0, 100], z in [zlow, zhigh].
#for m, zlow, zhigh in [('o', -50, -25), ('^', -30, -5)]:
#    xs = randrange(n, 23, 32)
#    ys = randrange(n, 0, 100)
#    zs = randrange(n, zlow, zhigh)
#    ax.scatter(xs, ys, zs, marker=m)

#ax.set_xlabel('X Label')
#ax.set_ylabel('Y Label')
#ax.set_zlabel('Z Label')

ax.set_aspect('equal')
plt.axis('off')
plt.show()
