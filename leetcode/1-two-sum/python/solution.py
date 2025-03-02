from types import List


class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        hm = dict()

        for i in range(len(nums)):
            c = target - nums[i]

            if c in hm:
                return [i, hm[c]]

            hm[nums[i]] = i

        return [0, 0]
