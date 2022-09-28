import functools

class PureList:
    def __init__(self, car, cdr):
        self.car = car
        self.cdr = cdr
    
    def __iter__(self):
        return wrap(self)

def cons(x, xs):
    return PureList(x, xs)

def zip_with(xs, ys, f):
    return PureList(mem(lambda: f(xs().car(), ys().car())), mem(lambda: zip_with(xs().cdr, ys().cdr, f)))

memd = {}

class memorized:
    def __init__(self, f):
        self.f = f
        self.m = None
    
    def __call__(self):
        if self.m is None:
            self.m = self.f()
        return self.m
    
def mem(f):
    if memd.get(id(f), None) is None:
        memd[id(f)] = memorized(f)
    return memd[id(f)]

@functools.lru_cache()
def fibs():
    return cons(lambda: 1, mem(lambda: cons(lambda: 1, mem(lambda: zip_with( fibs, lambda: fibs().cdr(), lambda x, y: x + y )))))

class wrap:
    def __init__(self, l):
        self.l = l

    def __next__(self):
        a = self.l.car()
        self.l = self.l.cdr()
        return a

def main():
    from itertools import islice
    print(list(islice(fibs(), 100)))

if __name__ == '__main__':
    main()
