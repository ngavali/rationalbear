//https://leetcode.com/problems/palindromic-substrings/

struct Solution;

trait FindPalidromeStrategy {
    fn count_substrings(&self) -> i32;
}

struct UsingDp {
    s: String,
}

impl FindPalidromeStrategy for UsingDp {
    fn count_substrings(&self) -> i32 {
        let s = self.s.as_bytes();
        let mut dp = vec![vec![false; s.len()]; s.len()];
        let mut palindromes = 0;
        for i in 0..s.len() {
            dp[i][i] = true;
            palindromes += 1;
        }
        for i in 0..s.len() - 1 {
            if s[i] == s[i + 1] {
                dp[i][i + 1] = true;
                palindromes += 1;
            }
        }
        let start_len = 3;
        for palindrome_len in start_len..=s.len() {
            let mut i = 0;
            let mut j = i + palindrome_len - 1;
            while j < s.len() {
                dp[i][j] = dp[i + 1][j - 1] && s[i] == s[j];
                if dp[i][j] {
                    palindromes += 1;
                }
                i += 1;
                j += 1;
            }
        }
        palindromes
    }
}

struct ExpandFromCenter {
    s: String,
}

impl ExpandFromCenter {
    fn get_palindromes(&self, mut idx: (usize, usize)) -> i32 {
        let s = self.s.as_bytes();
        let mut palindromes = 0;
        while idx.1 < s.len() {
            if s[idx.0] != s[idx.1] {
                break;
            }
            palindromes += 1;
            if idx.0 > 0 {
                idx.0 -= 1;
            } else {
                break;
            }
            idx.1 += 1;
        }
        palindromes
    }
}

impl FindPalidromeStrategy for ExpandFromCenter {
    fn count_substrings(&self) -> i32 {
        let s = self.s.as_bytes();
        //let mut dp = vec![vec![false; s.len()]; s.len()];
        let mut palindromes = 0;
        for i in 0..s.len() {
            //println!("Odd->  {i} {i}");
            palindromes += self.get_palindromes((i, i));
            //println!("Even-> {i} {}", i + 1);
            palindromes += self.get_palindromes((i, i + 1));
        }
        palindromes
    }
}

struct StrategyBuilder;
impl StrategyBuilder {
    pub fn build(name: &str, s: String) -> Box<dyn FindPalidromeStrategy> {
        match name {
            "UseDp" => Box::new(UsingDp { s }),
            "ExpandFromCenter" => Box::new(ExpandFromCenter { s }),
            _ => unimplemented!(),
        }
    }
}
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let stratgy: Box<dyn FindPalidromeStrategy> = StrategyBuilder::build("ExpandFromCenter", s);
        stratgy.count_substrings()
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use super::Solution;
    fn testcases() -> Vec<(String, i32)> {
        vec![
            ("abc".to_string(),3 ),
            ("aaa".to_string(), 6),
            ("a".to_string(), 1),
            ("tlthpowwythupxaszmxhqbfbxegiqzdxzesppfjgycyprjyscubntihrfwbeebqgeclzdccxwxezasfzclndmnfjjqoplbxuygtopqtnpatixyydboldybmdoyfwkevwgxmsrdkwjiyoksilsorcbotqitujdaavjbvrjjwtnimpldrnkfuftxnhzfiwzkhslzolbfmdkqhulpjxmbmzqhawiztcbbaggcccttokwkznsctemmdgutldvpybalridbjmupbjijmexzjuvdfntqxuvdoijbbmbpwhbtdbxlhrvbxrfcoyjbrfsowdamohdsoojivwgoopfjwzdjhlzelkdxpsrkfdkjktptahoeanuuuujdybotiitmttzpnbyrxtjetxhydhlvlsmveddtvaobbvdkwxzoyugojhoapbaghgcanazkpauaorgjluszsezbnaqjxtvoxfitnpwsmywlxdimemymuyehyabrpedfkgrwtgyjvzkvahcbekqsofcvseswvtdixaxrjwjinvrruoskqlxcnxrvaqvsnpxdwjpjaupdyfaaxqsnrcrmkmzmtnndniqxglucqwargfzzqwxvaopxwafbzuifptayzoedznsljslpaoytiqnnlxeegbebslvbbsfoqlbokxakkaxdqyttxkdermidtoxjnjwibnlrsuvdkfcvoeagzpsogmcoeihbvyvjcdirnbbpqhdgoirclqapqzsvuesezbhdjoumxwhtwwnxnwyrnyhpaeqzbirnqxsufritrjkgbftmnjwpoakrzokpopwmwjsimwkvblwplsammgwonxrdkfbongodjnvadspxuvcyxlwvwhonvagznjsslbfayoxpqwrizxdhwgskewymhdlurtbekqsmghgzufkmsvrchskdoudtllfflromzwwahigprsrydcsyionczumedayyvldefctkuzafmwsvbifaxzyqywhzpqbeun".to_string(),1089),
            ("vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv".to_string(),500500),
            ("mmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh".to_string(),250500),
            ("wbsphbopkiurpbducxsusqqrtqivsgmidqwuhdkrpuayyuppyzthumzgjfrpbjabnknomypinjcvrqdsldxnrxyneeiaiwjfuxfgrleksmlciqdwsnskvcxxecdwkfcmuwjqeypouwmwdooarnhrayotofsgtpwpihlvodrxwbypwwczpesvxiahpkktknizavgfkvuqmwgzgppjjvvyxhlwcawqdnrtpmakrvhkxcjozirolthofuvlogqyjtwdhtmqlglbqmaeclfzqooztuujycudlkyzlmlwpjvthlzzsynvwdloakpkyabyeesblrsbhcivqrczinnbztnwiyfufjhqcfvoicqajllqxyaqwqlslwlylolhyqawcnjpvvsneyuryefyjxhwcehrzvytuqpdbvkxwbpebilakqikkcqtkyerlmhtkmgixwaebebonfeapaiazgxgbbxaidrockcefeckcordiaxbbgxgzaiapaefnobebeawxigmkthmlreyktqckkiqkalibepbwxkvbdpqutyvzrhecwhxjyfeyruyensvvpjncwaqyhlolylwlslqwqayxqlljaqciovfcqhjfufyiwntzbnnizcrqvichbsrlbseeybaykpkaoldwvnyszzlhtvjpwlmlzyklducyjuutzooqzflceamqblglqmthdwtjyqgolvufohtlorizojcxkhvrkamptrndqwacwlhxyvvjjppgzgwmquvkfgvazinktkkphaixvsepzcwwpybwxrdovlhipwptgsfotoyarhnraoodwmwuopyeqjwumcfkwdcexxcvksnswdqiclmskelrgfxufjwiaieenyxrnxdlsdqrvcjnipymonknbajbprfjgzmuhtzyppuyyauprkdhuwqdimgsviqtrqqsusxcudbpruikpobhpsbw".to_string(),1517),
            ("wjwjnyyecpwqayghzctgskythyjmrvpgashdbzzthhtzjssddjdgbtswzvteoexngowmfpngrvslggsqcappibwvzjafdwzneocnlhjveeqgilbdoflveuaysghzwnuffjzgayiljrkmlnkrfxfuslswkrxtkmalfgkslnovasfvkcdpjaublmexiguigdteypcnqzhycxdraibxprxvvxvotpjgvghpuhfpbmffhppubpluhcbgalyyrxcgdccvxuvredwghgcmqqvkqeusqogpxmtntsqjaiaulimmxgwxnbhaydkmeeyppjnyrrytfbdrbemrhsusinslrgsavqzyiwuyrzmglfyozsxhvsjivrvecnefoamnstvfxcaijxipvxfigoiwhiyjqokntfwdqnneexzvthuulxgcbhjntawurbastlflmsgfddunnpxibqwalviwjkcejskrkiwhgdqswemmbhsauzcxtcegksisyorggroysiskgectxczuashbmmewsqdghwikrksjeckjwivlawqbixpnnuddfgsmlfltsabruwatnjhbcgxluuhtvzxeennqdwftnkoqjyihwiogifxvpixjiacxfvtsnmaofencevrvijsvhxszoyflgmzryuwiyzqvasgrlsnisushrmebrdbftyrrynjppyeemkdyahbnxwgxmmiluaiajqstntmxpgoqsueqkvqqmcghgwdervuxvccdgcxryylagbchulpbupphffmbpfhuphgvgjptovxvvxrpxbiardxcyhzqncpyetdgiugixemlbuajpdckvfsavonlskgflamktxrkwslsufxfrknlmkrjliyagzjffunwzhgsyauevlfodbligqeevjhlncoenzwdfajzvwbippacqsgglsvrgnpfmwognxeoetvzwstbgdjddssjzthhtzzbdhsagpvrmjyhtyksgtczhgyaqwpceyynjwjw".to_string(),1590),
        ]
    }

    #[test]
    fn test_count_substrings() {
        for (s, want) in testcases() {
            assert_eq!(Solution::count_substrings(s), want);
        }
    }
}
