package main

func isPalindrome(s string) bool {

	i, j := 0, len(s)-1

	for i < j {
		l := s[i]
		r := s[j]
        if (96 < l && l < 123) {
            l -=32
        }
        if (96 < r && r < 123) {
            r -=32
        }
		if (47 < l && l < 58) || (64 < l && l < 91) {
			if (47 < r && r < 58) || (64 < r && r < 91) {
				if l != r {
					return false
				} else {
                    i++
                    j--
                }
			} else {
                j--
            }
		} else {
            i++
        }
	}

	return true
}
