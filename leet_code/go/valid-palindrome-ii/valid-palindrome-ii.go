package main

func validPalindrome(s string) bool {

	i := 0
	j := len(s) - 1
	dropped := 0
	for i < j {
		if s[i] != s[j] {
			dropped++
			if dropped > 1 {
				break
			}
			i++
		} else {
			j--
			i++
		}
	}

	if dropped > 1 {
		dropped = 0
		i = 0
		j = len(s) - 1

		for i < j {
			if s[i] != s[j] {
				dropped++
				if dropped > 1 {
					return false
				}
				j--
			} else {
				i++
				j--
			}
		}

	}

	return true
}
