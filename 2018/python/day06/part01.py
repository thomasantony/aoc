import sys
import ast
import itertools as it
from scipy.spatial import ConvexHull
import collections as cl
import numpy as np
from scipy.spatial import Delaunay



def pairwise(iter):
    a, b = it.tee(iter)
    next(b, None)
    return zip(a, b)


def manhattan(p1, p2):
    return abs(p2.y - p1.y) + abs(p2.x - p1.x)


def manhattan_circle_iter(pt1, radius):
    """Returns iterator with points inside manhattan circle centered on `pt`"""
    return it.product(range(pt1.x - radius, pt1.x + radius),
                      range(pt1.y - radius, pt1.y + radius))


def get_closest_point(pt, points):
    return min(points, key=lambda p: manhattan(p, pt))


Point = cl.namedtuple('Point', 'x y')
raw_points = []
for line in sys.stdin:
    point = ast.literal_eval(line)
    raw_points.append(Point(*point))


hull = ConvexHull(raw_points)


outer_points_set = set(raw_points[pt] for pt in hull.vertices)
inner_points_set = set(raw_points) - outer_points_set

outer_points = sorted(outer_points_set)
inner_points = sorted(inner_points_set)
sorted_points = sorted(raw_points)

min_x = sorted_points[0].x
max_x = sorted_points[-1].x
min_y = min(sorted_points, key=lambda pt: pt.y).y
max_y = max(sorted_points, key=lambda pt: pt.y).y
range_x = max_x - min_x + 1
range_y = max_y - min_y + 1

index_grid = np.ones((range_y, range_x), dtype=np.int32)*-1
dist_grid = np.ones((range_y, range_x), dtype=np.int32)*-1


points = np.array(sorted_points)
tri = Delaunay(sorted_points)

for tri_idx in tri.simplices:
    tri_pts = points[tri_idx]
    print(tri_pts)
    a, b, c = tri_pts
    # O = x, y
    


# def update_distances(sweep_x, left_points, index_grid, dist_grid):
#     """Update distances for points to left of sweep line."""
#     for x in range(sweep_x):
#         for y in range(0, index_grid.shape[0]):
#             grid_pt = Point(x, y)
#             for i, pt in enumerate(left_points, 1):
#                 dist_line = manhattan(grid_pt, Point(sweep_x, y))
#                 dist_pt = manhattan(grid_pt, pt)
#                 dist_grid[y, x] = min(dist_pt, dist_line)
#                 if dist_pt < dist_line:
#                     index_grid[y, x] = i
#                 elif dist_line <= dist_pt:
#                     index_grid[y, x] = 0
#
# sweep_i = 1
#
# print(sorted_points)
# # Sweep line from Fortune's algorithm
# for sweep_x in range(min_x, max_x + 1):
#     print('sweep_x', sweep_x, 'current_pt', sorted_points[sweep_i])
#     if sweep_x > sorted_points[sweep_i].x:
#         sweep_i += 1
#
#     left_points = sorted_points[:sweep_i]
#     update_distances(sweep_x, left_points, index_grid, dist_grid)
#     if sweep_x == 3:
#         break
# print(index_grid)
# print(dist_grid)
