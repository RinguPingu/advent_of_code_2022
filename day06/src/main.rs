use std::marker;

fn main() {
    // let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let input = "pnnfhnhshrhmhwwmwzmznmnwmwfmfhfjfcjjtgtbggpdgdjjbjrjsjpjrrmddmgmpmddrhddnfnfzfpfvpfpprhhlffmtffqhhdtdcdsswsdwswmmfvvpdprrnnhhhtffnfbbznbznnvdnnbffjrfrbfrbrgbrrntnggrqqwtqwwgjgsswgwqwtwwsvwvbwvwrwlrlppzfzwfzzpmzzhqqzqlzlglzzmrmwrmwwvmwvvnppjfjttlffhjjjsccbggnffqgfgjjnccmdmzmllvnlnznttlvlttvnvgnvvqvmvqqzrqqcgglzzwtztwwmjmzjjnddsffqrqlrrvsvdvldvvlgvlvccdzczcqcpphggtnthhhtbhtttcjtjcjgcjcbbrhbbfrffgjgdgzddcttczzsccbpcpddcpcggmjgjddtcccthccfrccmdmhmddnwddfldffntnptnpttcptcptpfphhmfmwfwmmlblgbgvbvlltqltldttfcfcclgcllmplmlbbjnjzjnzzttnvvgddshddsqddggsqgqddsggdhghjgjhhgchhdmdjmjddgdhghrrphrrpnnqhhjwwqrrcmmslmmszzpgzpzzrmzmznzllnjnnlnbbdvdsdffbpffcmmnznqqcbbzvvjnjvvwqwgqqpnnzwnzwnzwwwlpwpzzfqzfqqwnqnbnfnqnbqqbggqnqdndrrzzlffbbgqgfgrfrqqsddnqnqjjgssqwqwcqcpcrrqppwpjjfnfpffhphwwmcwwznwznnplptlplnlsnlsldslslsffwtwftwtbtdbbjsjcczwwfllwtlwtlwlvwlwrwppsggvcvrrqcrqqvmqvmmbrrsbsfspspjpnnmpmqmcczgzffqmfmtmpppwzppzrpzrzsrrpqrpqpmpvmpvpttbqtqmtmjttqdqgggcppclcjcpcsppctpplpdpcppcmmdzmzddvhvhnhrrldllcwwbnwnssshlhrhthggtmggbjbjwwbvbttjllvrrfggngvngnmmvzzrrmddmcddztztctfccqpcpcqqvqppqcqdcdhhvhssgfgzgwwzmmnssvwwbqbhbnhnphpqqjcqcddfwfttqjtqtlttglljgjbgjgnnsqqvrvffqvqfqbffljjpffssqdsqstqqqldqqmhmsmsqqwtqwqdqgdgjjfbjffgbbrhhqghgppqgpgmpmmfzfhhfrhffgmfggpzgpzzhtzhhlbhhqbbzvvnvqnntptmmbhbdhdwwmjjcnnmsscqcbbtjtvjvwjvvmsmjmtmpmgghttcztzggpddbfbfgbbdsdrsrrqfqjjqfjjhzhtzzmdzzcgzgdzzmvmmfmjmgjmggmppbdppmzpppvfppzhhfsfwfhhpjpmmrjjpssdccjpjpwjppvdpphcpcjcfjfwjfwfjwfjjqcqcwqqqsmmmbbgdgwwpcwwdfdlflrltlgtthfhfjhhlthtddlgdggsjsrrdpdcppgttphpgpwppmpzmmrjmmvjvgvgfglfgllbqlqhllszzlwwhzzdfdcdtctptwtztfzzmjzjtjtrjrcrnrjjmwmnnbddgvgtgsstjsszmpqdmzgqflrbrspjmtzjcrmlzltmhgblghnwqvwwqwzbpnfrpdpblpjgshfccfbjfsnwvvhnjftsdnsgtzzjtzpmtfdvzrhtqpblhwgmqtgpbfvbdmsnrrrvvbstpsznvbbwgjfqjrhdvwvgptpglpfddhddmtglmjlpwlvfpbtbmgbplbzrlpdlvqzcwhbscpszgfstjpfdvfpmljlngrbgrdnnblzqrfpzsdvblpwbtnhdjclldvwvbwcwzfzbdspgwpfqjfbdbrqcshtlvcrdstnzggbwqnzbrfzbpnrtmvpbvdhcvdsdshgtvhfgdzljflppqbwclnvbhbczvrscjhlbgbfvwdjhnjsgmvwhpfgwbbmnndpnglfrmtfdzvqgfjdqfhgrhvpbqndmqnqccgwswwdsqjnbjtjbjdbqgjnmfbdvlnfwbnrdqgvgzzhmmbbdzfdvvpwhpbwbnzdcdpchrwlhfsjnhhjggvplmqggwjdsvjtpnpnqgldjjdcscrdltssjdrpcrfbgbcjfplhzgwbprfcslhpcngtszrghmwhzdqscbfrhzdwcffzvmjrmcjcstfvhplvrsglgsjnjtrpddsdfqjsndjnfmvdhfgdbzzflqhsrrwmrnlpqzmcddqbqvvzgtlztpgjnddtcnbmqsjlhmcszrmcjvwzpptlfqsmpvgnzvrjdwzpdwqgbmdgdtvjlmfczthjbcgfhbqpnmlbmrwwhfptzlbmfdhssznjcvjbmnjtnvzjhzczlrrdnttmmcbnzhqpplzqwgttwrnwfvmnptgqlfrnzvqpjfgrzwmlcwvtptvcvrlsrdwdgqfvffspmdbnnrqjttpqvhvdpbcrvzptwnhhfsqzchmncvttcdgdnlppcfzpmjpvbvqhlvplwvrmmbbggbwttwmvsqjlllsftprsmtmnzjcqfzblrllzgshfljchrjwjlpvhpbrtrsschzltrblgjnbgdnmwdggjhqggntblnhsvfgsbcblhmctbqzqwmhqnjhpzjfqpjdgwpzhczcftfcpdhvzhzccmwmrfrbqshzmtpqgpbbvfqqbjbmvnlnlwjtzrpmhdlffccrqcfgsjfszbrzrfztntchtmgmbhjgmlsqzcbtqqjzzlghtzzqmlnnvsgsvbbjfgqsqbqmqrdzwpwdgbggpdvhvnlzshhntprjdwhnwfvdjzpqgflwrvwgtmfdmfdztcbtfnjdrvgdwwczdgphnvdgrbdchprqldfjrvcsflcmlcmzqvqgsgnzcgmrhccgcmptcdzhbcdgdtppwztfstzqqzqrdzlnzthggjmpcflmbcmdrrjnnpbpqfmjbzqbtsjjgdlmgncbmgspqqvbrvzrdjscpzjsdtcdvsdwqlmwrngttswnrsbqctvhgfnnwblpcqzdmzpfchplslspmghvgcqntmlrfhgpcbpspvfhnvqvglsqzsnsdzddqpbsjhlclslngbwvvgjhwfcncqsmqwbptzvpzlzslsjjjldjpwpfrdlfbjphqcjtsgqdsdfdjhqgdhcppndwmhmmldvvmblcqcqfqhltbcbvrnghjfmtgqwtwljtczvqlnmgscjhqdhnzwhzvzzqnlsrhqvljqpgpwghfqlhjjrrhvnmnnrbnlhdcjctwtlhmhhmhjvcgzdrzmdjrvqzgnsttjdwglgwlcmbcdnjprgfsbbdzzngbqdrvwwwhbtlnnmzqdjttsrrpvlfdqnfhhtdtvmpcjgdwtbnqmwmtszdqfmbhjsjpqqddzfggwjhbtlnqfgcwbjzdtcpcpzgnrmnvwlpgmwfjlpgppdfrfvvjwsfcdqdnpcpjbqsvhttssgptqjghctrbgntlfjzdrfjccsprsjlrrwrzsmnjsqslmpdtrvhlqbnmgpjthpqdqmnvrtzlhhzzfzbrcclpmpcszhbttgrtcpgcpjwpdbfpfvgspsgtvglwthqcmcvmrfmclwlvjlsptfgmtlrnsvjrnfwzhdcsmgztpzfcvzwdztpppvqpvqfpdrsfnlhrbqwrsqjtwjmhnpwmqmpdgdhbtbpfwnmswffdqffdggrdrpmngvpzplmmwlddnhcvjjzqqfsbbtfmzdwnpvbjrshmllczhgvwwcbcbtfrfnplqjwmjlvpwwgfrtffwddwppsgtnlmpvfnhfzcsgjbqbjmbvpnqppsrvwnlzvcmjqgtbzrdsnrgwbfmrvnflgccrssfvcwgllqqbbcthzmbtnsmbzbcczhtzcvmthttpltrtdmgspctvtpvqbhmnnpnjwmhpqclmjsdrbjwvjbtzcjlqbjsvbgdwqzflnwzcfjwtrhjgfshfmwbjfwpnhjsmtpgbpwlfjjnmdlrhchmnfmgmgcrftmwbzshdwbhndgwtjbrrvbwprqppfmgfmfllpcjgrwdmtzddthsjlgjljv
";

    let marker_length = 14;

    let mut received: Vec<char> = Vec::new();

    for c in input.chars() {
        received.push(c);

        if received.len() >= 4 {
            let mut four: Vec<&char> = received.iter().rev().take(marker_length).collect();

            four.sort();
            four.dedup();

            if four.len() == marker_length {
                println!("{:?}", received.len());
                break;
            }
        }
    }
}
