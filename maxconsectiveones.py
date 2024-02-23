class Solution:
    def findMaxConsecutiveOnes(self, nums: List[int]) -> int:
        max_consec = 0
        current_consec = 0 
        for i in range (0, len(nums)):
            if nums[i] == 0:
                current_consec = 0
            else:
                current_consec = current_consec + 1
            if current_consec > max_consec:
                max_consec = current_consec
        return max_consec