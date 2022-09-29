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
