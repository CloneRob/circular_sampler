import numpy as np
import matplotlib.pyplot as plt

from math import pi,sqrt

def getKey(item):
    return item[1]

def calc(start_id, x, y, t):
    distances = []
    for j, d in enumerate(zip(x, y)):
        distances.append((j, sqrt((x[start_id] - d[0])**2 + (y[start_id] - d[1])**2)))
    distances = sorted(distances, key=getKey)
    indices = []
    for ind, dist in distances:
        if dist >= t:
            break
        indices.append(ind)

    indices = sorted(indices, reverse=True)
    x_n = 0
    y_n = 0
    for ind in indices:
        x_n += x[ind]
        y_n += y[ind]

    x_n /= len(indices)
    y_n /= len(indices)

    return x_n, y_n, indices

def main():
    n = 6000 
    res = 550
    t = 35

    theta = np.random.uniform(high=2*pi, size=n)
    rho = np.sqrt(np.random.uniform(size=n))
    print(np.max(rho))
    print(np.min(rho))

    x = np.cos(theta) * rho
    y = np.sin(theta) * rho

    x = res * x
    x = x.tolist()
    # x = x.astype(int)

    y = res * y
    y = y.tolist()
    # y = y.astype(int)

    plt.plot(x, y, "o")
    plt.show()

    x_centroids = []
    y_centroids = []

    counter = len(x)
    while counter > 0:
        xn, yn, indices = calc(0, x, y, t)
        x_centroids.append(xn)
        y_centroids.append(yn)
        for ind in indices:
            del x[ind]
            del y[ind]
        counter -= len(indices)

    x.extend(x_centroids)
    y.extend(y_centroids)

    print(min(x))
    print(min(y))

    plt.plot(x, y, "o")
    plt.show()

    print('done')
    print(len(x))


if __name__ == "__main__":
    main()
