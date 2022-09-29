class Solution:
    def longestCommonSubsequence(self, s1: str, s2: str) -> int:
        if len(s1) == 0 or len(s2) == 0:
            return 0

        dp = [[0] * len(s2) for _ in range(len(s1))]

        for (i, x) in enumerate(s1):
            for (j, y) in enumerate(s2):
                if x == y:
                    val = 1 if i == 0 or j == 0 else (1 + dp[i-1][j-1])
                else:
                    val = max(dp[i-1][j], dp[i][j-1])
                dp[i][j] = val

        return dp[-1][-1]

