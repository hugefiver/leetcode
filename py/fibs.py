from itertools import chain, islice, starmap
from typing import Iterator

class CachedIter:
    iter: Iterator
    cache: list

    def __init__(self, iter):
        self.iter = iter

    def new_iter(self):
        class Inner:
            def __init__(innerself):
                innerself.idx = 0

            def __next__(innerself):
                if innerself.idx < len(self.cache):
                    ret = self.cache[innerself.idx]
                else:
                    ret = next(self.iter)
                    self.cache.append(ret)
                innerself.idx += 1
                return ret
        return Inner()

    def __iter__(self):
        self.new_iter()

def fibs(n):
    def baz(iter):
        chain([1, 1], starmap(lambda x, y: x + y, zip(iter, islice(iter, 1, None))))
    i = CachedIter(baz(i)) # ...
    list(islice(i, n))