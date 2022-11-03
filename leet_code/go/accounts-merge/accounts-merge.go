package main

func accountsMerge(accounts [][]string) [][]string {
 
    nameMap := make(map[string][]int,len(accounts))

    for i := range accounts {

        if indexes, ok := nameMap[accounts[i][0]]; ok {
            for _, idx := range indexes {
                emails := accounts[idx] 
                for _, email := range emails {
                    for _, e := range accounts[i][0] {
                        if email == e {
                            
                        }
                    }
                }
            }
        }

    }

    return accounts
}

func main() {

}
