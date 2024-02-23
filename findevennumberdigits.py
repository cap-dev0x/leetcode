class Solution:
    def findNumbers(self, nums: List[int]) -> int:
        even_size = 0
        for number in nums:
            temp_num = str(number)
            if len(temp_num) % 2 == 0:
                even_size = even_size + 1
        return even_size