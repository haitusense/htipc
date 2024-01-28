import htipcPyo
import numpy as np

# htipcPyo.namedpipe("SimpleGui", "draw", json = True)

list = [0, 1, 2, 3, 4, 5]
m = np.array(list).reshape((2, 3))
print(m)
list2 = np.ravel(m)
print(list2)
