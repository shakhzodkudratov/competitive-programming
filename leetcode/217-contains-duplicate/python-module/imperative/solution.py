from typing import List


class Solution:
    def containsDuplicate(self, nums: List[int]) -> bool:
        hashMap = {}
        for i in nums:
            if i not in hashMap:
                hashMap[i] = True
                continue
            else:
                return True

        return False
