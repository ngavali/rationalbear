package main

func isValid(s string) bool {

	parenthesesMap := map[byte]byte{
		']': '[',
		'}': '{',
		')': '(',
	}
	stack := []byte{}

	for i := range s {
		if v, ok := parenthesesMap[s[i]]; ok {
            if len(stack) > 0 {
			    if v != stack[len(stack)-1] {
				    return false
			    }
			    stack = stack[:len(stack)-1]
            } else {
                return false
            }
		} else {
			stack = append(stack, s[i])
		}

	}

    if len(stack) != 0 {
        return false
    }
	return true
}
