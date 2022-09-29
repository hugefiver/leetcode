from functools import cache


@cache
def lcs(s1, s2):
    if len(s1) == 0 or len(s2) == 0:
        return ""

    if s1[0] == s2[0]:
        return s1[0] + lcs(s1[1:], s2[1:])
    else:
        x = lcs(s1[1:], s2)
        y = lcs(s1, s2[1:])
        if len(x) > len(y):
            return x
        else:
            return y

def lcs(s1, s2):
    @cache
    def baz(i, j):
        if len(s1) == i or len(s2) == j:
            return ""

        if s1[i] == s2[j]:
            return s1[i] + baz(i+1, j+1)
        else:
            x = baz(i+1, j)
            y = baz(i, j+1)
            if len(x) > len(y):
                return x
            else:
                return y
    return baz(0, 0)