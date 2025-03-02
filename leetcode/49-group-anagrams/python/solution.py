from types import List


class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        hm = dict()

        for s in strs:
            ss = "".join(sorted(s))

            if ss in hm:
                hm[ss].append(s)
            else:
                hm[ss] = [s]

        out = []

        for a in hm:
            out.append(hm[a])

        return out
