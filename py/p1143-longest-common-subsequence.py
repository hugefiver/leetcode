from functools import cache


class Solution:
    def longestCommonSubsequence_1(self, s1: str, s2: str) -> int:
        @cache
        def baz(i, j):
            if len(s1) == i or len(s2) == j:
                return 0

            if s1[i] == s2[j]:
                return 1 + baz(i+1, j+1)
            else:
                x = baz(i+1, j)
                y = baz(i, j+1)
                return max(x, y)
        return baz(0, 0)

    @cache
    def longestCommonSubsequence(self, s1: str, s2: str) -> int:
        if len(s1) == 0 or len(s2) == 0:
            return 0

        if s1[0] == s2[0]:
            return 1 + self.longestCommonSubsequence(s1[1:], s2[1:])
        else:
            x = self.longestCommonSubsequence(s1[1:], s2)
            y = self.longestCommonSubsequence(s1, s2[1:])
            return max(x, y)
