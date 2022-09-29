from collections import namedtuple


def lcs(s1, s2):
    if len(s1) == 0 or len(s2) == 0:
        return ""

    Found = namedtuple('Found', ['max', 'x', 'y'])
    dp = [[0] * len(s2) for _ in range(len(s1))]
    found = Found(max=0, x=0, y=0)

    for (i, x) in enumerate(s1):
        for (j, y) in enumerate(s2):
            if x == y:
                val = 1 if i == 0 or j == 0 else (1 + dp[i-1][j-1])
            else:
                val = max(dp[i-1][j], dp[i][j-1])
            dp[i][j] = val
            if val > found.max:
                found = Found(val, i, j)

    if found.max == 0:
        return ""
    else:
        xs = []
        i, j = found.x, found.y
        while len(xs) < found.max:
            if s1[i] == s2[j]:
                xs.append(s1[i])
                i -= 1
                j -= 1
            else:
                if i > 0 and dp[i][j] == dp[i-1][j]:
                    i -= 1
                else:
                    j -= 1
        return ''.join(reversed(xs))
