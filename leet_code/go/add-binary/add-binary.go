package main

func addBinary(a string, b string) string {

    if len(a) < len(b) {
        a, b = b, a
    }
    var sum, carry byte
    var res = make([]byte, len(a)+1, len(a)+1)
    
    for i := range a {
        sum = 0
        bit_a := a[len(a)-i-1] - '0'
        if len(b)-i-1 >= 0 {
            sum = b[len(b)-1-i] - '0'
        }
        tmp := sum
        sum = sum ^ bit_a ^ carry
        carry = ( carry & tmp ) | ( tmp & bit_a ) | ( carry & bit_a )
        res[len(res)-1-i] = sum + '0'
    }

    if carry == 1 {
        res[0] = carry + '0'
        return string(res)
    }
    return string(res[1:])
}
