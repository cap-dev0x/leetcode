class Solution(object):
    def isValid(self, s):
        stack = [] # create an empty stack to store opening brackets
        for c in s: # loop through each character in the string
            if c in '([{': # if the character is an opening bracket
                stack.append(c) # push it onto the stack
            else: # if the character is a closing bracket
                if not stack or \
                    (c == ')' and stack[-1] != '(') or \
                    (c == '}' and stack[-1] != '{') or \
                    (c == ']' and stack[-1] != '['):
                    return False # the string is not valid, so return false
                stack.pop() # otherwise, pop the opening bracket from the stack
        return not stack # if the stack is empty, all opening brackets have been matched with their corresponding closing brackets,
                         # so the string is valid, otherwise, there are unmatched opening brackets, so return false

# Static test cases
test_1 = "()"
test_2 = "()[]{}"
test_3 = "(]"
test_4 = "]]]]]"

solution_instance = Solution()
result_test_1 = solution_instance.isValid(test_1)
result_test_2 = solution_instance.isValid(test_2)
result_test_3 = solution_instance.isValid(test_3)
result_test_4 = solution_instance.isValid(test_4)

print(result_test_1)  # Output: True
print(result_test_2)  # Output: True
print(result_test_3)  # Output: False
print(result_test_4)  # Output: False
