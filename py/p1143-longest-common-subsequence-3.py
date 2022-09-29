from bisect import bisect_left
from collections import defaultdict


class Solution:
    def longestCommonSubsequence(self, text1: str, text2: str) -> int:
        text1_dict, nums, lis = defaultdict(list), [], []
        text1_set, text2_set = set(text1), set(text2)
        for idx1, char1 in enumerate(text1):
            if char1 in text2_set:
                text1_dict[char1].append(idx1)
        for char2 in text2:
            if char2 in text1_set:
                for num in reversed(text1_dict[char2]):
                    nums.append(num)
        for num in nums:
            insert_index = bisect_left(lis, num)
            if insert_index == len(lis): lis.append(num)
            else: lis[insert_index] = num
        return len(lis)