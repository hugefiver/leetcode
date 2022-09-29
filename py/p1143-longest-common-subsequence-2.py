from bisect import bisect_right
from collections import defaultdict, deque


class Solution:
    def longestCommonSubsequence(self, text1: str, text2: str) -> int:
        m, n = len(text1), len(text2)
            
        idx = defaultdict(list)
        for i, c in enumerate(text1):
            idx[c].append(i)
        tail = deque()
        for c in text2[::-1]:
            for i in idx[c]:
                if j := bisect_right(tail, i):
                    tail[j - 1] = i
                else:
                    tail.appendleft(i)
        return len(tail)

    def longestCommonSubsequence_2(self, text1: str, text2: str) -> int:
        m, n = len(text1), len(text2)
        lcs = [[0] * (n + 1) for _ in range(m + 1)]
        print(lcs)
        for i in range(1, m + 1):
            for j in range(1, n + 1):
                if text1[i - 1] == text2[j - 1]:
                    lcs[i][j] = lcs[i - 1][j - 1] + 1
                else:
                    lcs[i][j] = max(lcs[i][j - 1], lcs[i - 1][j])
        print(lcs)
        return lcs[-1][-1]
    
    def longestCommonSubsequence_3(self, text1: str, text2: str) -> int:
        m, n = len(text1), len(text2)
        prev = curr = [0] * (n + 1)
        for i in range(m):
            curr = [0] * (n + 1)
            for j in range(n):
                if text1[i] == text2[j]:
                    curr[j + 1] = prev[j] + 1
                else:
                    curr[j + 1] = max(curr[j], prev[j + 1])
            prev = curr
        # print(curr)
        return curr[-1]
        
    def longestCommonSubsequence_4(self, text1: str, text2: str) -> int:
        def dfs(i, j):
            if i < 0 or j < 0:
                return 0
            if (i, j) not in dp:
                dp[(i, j)] = dfs(i - 1, j - 1) + 1 if text1[i] == text2[j] else max(dfs(i, j - 1), dfs(i - 1, j))
            return dp[(i, j)]

        m, n = len(text1), len(text2)
        dp = {}
        return dfs(m - 1, n - 1)