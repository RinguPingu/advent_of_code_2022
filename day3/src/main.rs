#[derive(Debug)]
struct Rucksack {
    first_comp: Vec<char>,
    second_comp: Vec<char>,
}

fn make_rucksack(s: &str) -> Rucksack {
    let halves = s.split_at(s.len() / 2);
    return Rucksack {
        first_comp: halves.0.chars().collect(),
        second_comp: halves.1.chars().collect(),
    };
}

fn main() {
    #[rustfmt::skip]
    // let input = vec!["vJrwpWtwJgWrhcsFMMfFFhFp","jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL","PmmdzqPrVvPwwTWBwg","wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn","ttgJtRGJQctTZtZT","CrZsJsPPZsGzwwsLwLmpwMDw"];
    #[rustfmt::skip]
    let input = vec!["BzRmmzZHzVBzgVQmZLPtqqffPqWqJmPLlL","hpvvTDcrCjhpcrvcGGhfLHMlLtMCqflNlWPJlJ","hGjhncHhGnhbTHczBBZVVSbRwgSgRV","rWVQjPQQjGRWNSrWrPjcptwBpqqJBtJBBcZgMdtq","zzmmpzfTCFpTlMlJJwBgMlqMBt","TvLszpbhhTLmsnRQPDQGWDWRvQSR","zGzvLlGlQHLGBQZlhBWhdjRdmdWRcjPj","fTJNfTfNSRWPhjdjfj","pbsbVVnpSnbVTprnbqqrzvLLgQlGLPLHll","ZCCCsWvNvmsCsCmZLZmgMLRpQMhwQRpQRfphfprpTfpM","tlncPjzlndctbzcPPBcjwDphwrfGGDffbDRpDTGG","cdqnddwzqjNVWVLZZLZq","DTLbDbRrlQbwhhNrmmfwdt","zzMJMzjCjJJjvLjMPJpcgPpzfhHdfqWcqddwtwfqdttcNtdN","pJCzVpCvDZBLsVRQ","STzBBbJzRRBZBRTqCCsfZLtNNLClCsfh","jsQnnQjjHcvQFrcPwCmtLCNlvDfftfff","sGFscMQQMMpqzqbMbd","QlNDWGsjQjgQllWQsbtzqTJczTJcbFmmFJJP","MhLrhgLVndRmzJFzVqqJqP","pSLnMdwhwdRZRSwhLZwLhdGWQjlsgWjNQWWSvgBsWDlj","THjSRFSddTjdBTcPLcVVvVBw","GzWnWfndWfznDfsnsBsPVwVwPPLL","zNflzJWqqzQDdSStHNZNpFFtbj","FSzDmsFSFlDlBzqVjqHHjHHpVgHLbp","rTrTtTQQntRQnQJQgggHZttVgHLBLhZL","WTJJRRQCRRJTRdBCRdvRNDFSWFMPmDlPPSsNPSzS","WQldlMtMVQgVMQHnDGbHGGnRnQmD","rqcZPrCFjmHlbGjZ","zSScchqwchBzTzFzhhSlcCwNtdVWWJgsVdMtWNgNVWTJTd","lMZqjMWllrTTspjprWWSSwgWNSVNDmWGVwFwgN","cdCCdLHcnndHJnmCRntLBnRzDvFNtNDVzgSgwDgFNVzFVv","BRLcCCJCmJdcRhfjPPZphrlrPqlZ","GdGqcrrZGDrvDJJqJHcBvmFFgmFMMgMgBtMLTssLmF","NbPVPDlljPmTmsTj","VfQDhflCCRWdcrQwJvvnJv","RLcWgLCqqPQLcqZwzHgwmmrmmtgwTw","DhbhNrMpnJSDJwVTHmmTVnTTVj","lrsvblMDlcWcfQPQ","PVldlphHwGwJJGdjZZWsRbbsGsNWrWQbNbQR","SqcDvTmDLtfmSmtqppfqzTgTQBrRQsbCFWbNNFQFBBrRbLNb","MgtmTgtfpqVlnVddZMwV","BdmfmPBPSbSNdGSdvWrwcZrccZPPcZnH","jzzLsjsMRlQQVHwswvvZrCHrrT","VqhzVFzplFlpLwpMphLRQQVRmSqgbdGtNJBmNSmgGbtggSgt","DHVpNZjdZjFZWVFHpvFvzmlRzPnlfznFRz","lrTBTsBwwMbrrwLPPfwGmGzvRf","scrtMhMCtJBBBclbHdHttWZWDSqDSjHj","wzqsPmqsbsfqBwPMNRMMZcZmFFNtZM","CgCnhlvvLJgcRFNNBdCpWM","QnQrVgHSvVHjbjTGBbbTHb","HdrVrdqFDdZVmHgRmDRFHMnTdTssMGnLnPJLbPTbCs","SczlScjwcNzplNzQSSfjwQSrTGGsbTsnTCnGTMCMLMGGbN","wQlfjrhfhQFHqZhRZRtD","RsfJDGJvzPNcjpddSWJWMd","LLCbBCwCrCmVVnrmhQFmbVhdcdlWpjZzSpMdWSpcWczSBj","rrLCbTwnHTvzvNGT","wPhPhbCqqSCrtJDlqvlrJr","RVVZddLFRZZcQLvJJtzptlgPJp","TVQRZGVncFdTGWZdCNShHhfPNwwsWPwb","dzLVzPSgrgDDDCMSMLLPwFmdTTcsvmwNwjNsHcFF","nWBGntQfGNGBflWBBqlpRQGbWFvjwsbsFswbvTHjjbmHTc","tBNJBnGBflQnDPJrPhDgrPVg","VtWztWtqpqzWpWzqjNRjNpWTmrrmrSbnmJwSJwnMPrCSJVwM","sDHsBDhBdsBZGcHvLHDLhhCSnRSwCJMZrPbmnMbJSCSR","ccLRhgsLBdRsdHNTFFNNgqTglqzF","hztlmDhPhgPlPNNgmZMCbmwwQjcwjjwMjVCd","RSJRrRqnqQJFqvnTGrHCcHHCCHHbHHMcMvdM","qGJsnQTRsStsftPlhPNl","BFFBLPRCwsLwhlPlRmhcGGrbmmGjfNTTnp","VJMVpzgqggJnrjmjNcMjmT","VqdSZtQgZvtdzqHqHtVZdVQpCDWWFdwlRPDpWPPBCswlWD","fCWCsjPzcbzwRSzVTzhhDLqvdg","TmJtrNJrBLSLJqgS","ptNTQFHrZlnpFPwsWMbRjCpcjR","nJmQNCmbmlllmbClbfMLjMFqbGBsdLFq","ZcgTWcTnMqqMTBqF","tPgctSnPctZZgDWzZgQHwNmHlhlmzlQhlJlw","ZpTCwpffdslvgShCBhqhRz","FDMPnNFNmBPzvRPRBg","nNgMrnnDGjDmJMmnFdZTTsdsrZrslcwcQr","pTmczpCldcdDDnPttpvWSqbpJf","jgjRZMGHhGLgQrjvPWzPJgJvzStbbq","LGNLLNBBzcDFCBwwFC","nJTTqnrNvTzNMzzNfqrTPrJnwpwPpZpsHccZVsBRpcVHwpcp","bgDhgbghLWmFmStctVpZtBCVCCpfZp","LLSgLGSjggFGbSSbmMnrvqvzjfzTNrJrqM","RRpDmmPMTjwfGmJQgQ","WsNscdnvvdVZFVnnrZbjjflwljlbzfGFjQjq","NnLZsNnrrVVVcvdBLTPCPCRMwhPMBMPhCt","lbVvzngGJnVbJHpHtHNPpdSQvc","TsMBswFZsWMWBZMNwPtNNtRNHcNpSQ","CcZCTrZDsjZTsTsshWhrWrTnfgbLDfJzVVLVVlgfnzfVGV","JzTTRtJRZWmWjrMHCT","DDFGlLGcGlSSSLsFGBspPBmNMBHMghmWNmWjWCmWtH","nSSpnbsGlLDnpPsSSspFtVvffRQdVzqvvbqdfVQwRz","sMhzszlHHDsWbthHDqsbJjpLNtmjVJmVLLVLVLBp","nrTPrGwfPLdprzJzdL","wgPQcTGGzgccwCgnRwgRChFhlWSDqWWQMWhssSsMQl","NSNmwtpSpCpvMphCsr","PHcRGPLJMrsvzsqG","QHjbnRMcfbPbQZmlZgZlgBBQ","cPRPbhQjbQRdtPQdLqLHqzFZjCFCqLjC","mmfsnnwrfvwrfSNZFzHHLDCFNlLlqDlN","wsmrwswwGTffMrBnmQttJtcMZQQtPJPbZc","MvBPDDRRdnnvHPCHZLHZsFLL","rmJcbVqbcjWwWjQHLzTZFTHSzFrpsz","cmwllVqqGJbVVVmmqbQcmgRnRvGhGfgDRDZBBBvRdd","nMvMhMnvhnbTZWSSZgHmGJDFmmNDzBmbNmdGBN","rCsPLRCssRjrLLsrLlwRVrcNJQfDQfdBmmfNBGJNzmDPfB","CRjCpLltgtJgJJWq","jshCzJpjzTPpmCWvSlpfwHfSWglf","LQMMNMnHtDtLVRvwwgRWlldgWD","qHVrQNHVMFQtrrBBQMBcrrZsZbzCZhbbJZJsmmsmFPTC","JZQZnsQNMqTngZqJBVfBfPPVBNrwvfPw","SSmDstFjpDpCszDjcLLhrPVlGlrGGVBwrvwVPt","FSssFcLjFjbmFFCzjLcFLRDnMJnTHRnZZTdWqZZWnMnRnZ","GbHRHpldwGMpWhHpCMBlCbRdVSLhnqJLSrDPLPPLPDqVDrhh","gvjWWQvgZFtQFFNqLnVnDnSJzzztDD","ZccccfTsffHdWWdRWwsw","ClCtbHMlnnPPlszV","gSDWSLgWQWQJJNWqgtQjPsnfcdVcLVdVdzfzVzff","WQgqtFQgDgQSFqJhqhSJvNDRrZMZHwHMCbZhTpZbGHMTMG","pZJZlCQtHFhPfdNfCh","zcmLSVczwcMcLDNFHdLPhPWH","szvVVnBmnTGQtHTQ","RVVCNDlNGzlGZqHGHWqWhGqQwH","ZFLFTmpLvvmSqsbb","TrfpBfJpJMlnnNfNZD","qHHlDClHhltMqQsHDhHslGznwdTnzzwDGSdfnwGnwG","mZRNcNcLLPNPBFFbbPmLmbZFSCVfJJTVndVfSwnRzznfTwCS","CcCWFbbBLCWtgWgHjghqvv","TjbzlnlFmfqCFFVVCRWr","PhMcLpPDtMLpwPDvLPJbMhSgVCGqggVqQgCqCgCgSWvv","btbZbNZhJDJJhDtwtsTTTmBzzBBmlNlmHj","FqhjWtqlqmmsnFPTCvMCQMTTCjQd","pfffRfLpgrgGgzrNVzzpGVzRCdMCPJbwwcVMbQPCJVMVdbww","DGGDZRGrHggzSsFQnnWShmtH","vtHVVMMrvVMVrSHvLgvlHcZFCnRCZcccZtRRZfJFCJ","rdDjGsdTQDcNZfdncCRR","rBDsTwBbjbmbbQswswPhqVmmSvpVhlvvqMhHhh","vGBLrqMNvqSLBvvrNbllLHfwStWWtFttccjtRtjtcj","MhCDJmhMDzmcRRcjzWfztH","ZQDmDhVVCQbBVdVNMvvv","ptCtCzhWPWptnhVzzpGZbZTjTjVjFGjVFgVl","fQswRRffmRqZlgrqqFjjSgGg","HwsQDNNsDsmRLLHmffsfvHptBnhtzCvhWpZWBdhnMdCh","RlHzzTqczBPfbnvcpB","wVtNwpSZstppwwMsZhsdnLvnbtBBmbnLFFdnmF","WNQJMVWsZWwGJWhhSNrQzlgHrDCgQRHpCHrl","RrZWpJZRrZpdTGstlchLGGlLMd","NqjDPCQPnQCSvtMzSLhhjM","nQVQDDDDfwBwNCVCNVFNpWpgJgrRTmLTmTmgRTWF","SHMcrMHpcjGcjSrMMbvSvvSvwFTLJwJNtFGFWJNtDLFTLfWN","zqRnPfzQCRzqsmRPzznhszzLtLwQwwFTgWWLDLgWFTwTNQ","qVPZmRZhsCZPhZlRCqRRRCbfpccMBjvMVjdHjjMjSvdf","VVQdHwBZLVltlddtBczhrzvGcWWFRwgsFG","TDTTTqqTSSqjqnmTmPqPPmTmGhRszvsrzsjRsccgzrRzgWGF","DpJPqpWqHbZpllpt","cCSCFsnnZFnscDtNdJFJtJtdmb","VgBqBsqRrHtNdzmNrt","BGLLVVjRBsqPBfsGwPsMfSSZCSfTZTZQpSphfS","plCHCHlgglHHGpNbtngNrDvBDpfQDBQfZDfWZVrr","mTmMLhRfwhsLPQvQZDMZQBQWMB","cwsssmqRTFFfFgtbCtGl","LQPPrCPnMZwqtRMn","cWTSlJWlcplJdDTdGdpDlGcGgqmtwwZtqRrNRRmRdNZqmgNq","GSJcJSjsjTpsvWGWBHLLvVVBBBrFrzVz","NVPCSPMNDSNFVSWCsJJJmpGmZZGLLcpZLHGGtsHt","fwzlBBqghqvzqqlDrHbpHjZHmGZbLZrHLb","dnBgnDqQvwRnSnnFMFMP","BCbPsFFwCRHmDSBmWnvDDj","phhZVzdpVfQZphhZpRhSVnjmrcvvnrWtDrvWDS","TfQJMfLphMhJdfdzpQJRTPbwHHNlgbGwsTGgCP","ttWLlnnvnNnBBtlTqWlpvpndQdZsQQFssFDdsRFdVdRNFQ","jSgrScrbGZSGrrCGsFVMssFsPPFcDDMV","bzSmJbfCZCbzLwllflwqtvvw","zmFTJwFLPmzLztmjDzTJwfNrdFNrFppBSNRGNGdbrpBR","gqlhWQgsZMsvqMlMMvsvqsNlLbcdppbrRpdbbcSrrbbr","vssCgVgCsggZQZCgsnsqWgWvfJPDLwffwTPPmzTnjTPmPmwJ","SpcRTPQLBLWpNNzjmmwwwRrR","tGlfvGhfnbDlbqlChnfFMrwsmwNssTMHMHjFwv","ZlhtCtffCdWcZWZVVT","jTTCcWHWJNgCGTzTmnzrmnGn","BwRRbFvtvvQmJJFMpMJr","ZBBwLvqbBZsRsbVsZSqbcZdJjHHjhfPCJfJfHhgc","VrnDSvvrLrfTdTLGfdRp","zcJzmcFcHGfdGmWTVd","tHsMhwPVctccHFHFcbSDbbPjnNbBnbvBQB","QttWQwLTnLnWTtnffnLQSBFVjNvBjBFNgMdCsVWsjv","pDqcmmRPHqgVBddjvN","DcclzbcbPbJLnNTfnw","plRcpsZDGlGZvWvMCNcLtttq","SrfrwSjSVrSjwbmSrHzmHJCQQPQzqttNNQJMzJtqMW","wSHVnfHfWwwHWFVfSnfgmmRsslFZZDBBGZsZsDTdGRTp","qSFQSgQNgQBrBHHcrW","VTmjVJLTwlTmwTVmsMJMVlJmPvcbvvbCBbGBPjGvBbBGWcbb","DnJTZwmnZRhnpqNdWt","dTVHjZLLZDVCfVHtLDDjQbscjWbSJMJPjsbWWb","FnqrnmzzFllmsWwtsFtQMMFc","lmqzzzngGmlNNBqGllzlBNRvptHHpTCHpDLpgDZdgvHvDD","sdRZQbCfZTSTdlfTZCffccWPHPPcPPwLwctRnLWn","BBJDzFVgCDrCJrqDJJhqJVVMLPHwcctFwcWHHGLcwGwGHnWc","ghpJgqqjCZbQdZpd","tbcpzbHSszcHBgqHGZgJJJhhww","jfvdvRTffQQrrFCRFTnGwJRqNRZVpJGZLZggLh","nQTjTnMndlTdQFMvnrClCnpzmzDtbbmBbcPSzzlmmtzP","BqBqTCSTcqHsJHHM","WWPGVPLtzVgWtjWPGzVjzVGcbDhPsRbDcsbJwNRswRDRss","VQfWjfLFGWLjdFfVzTZZpJTpnmlTrSQlBl","jLNsZjqSHCsGdsmpsm","MvnVFzWMwMVWzfnVDwfBMfnnrCtdtPmPlRrdrJCJrtPDrrPD","zznfFWwMfMfFMwVTMQFnQjhjgjSZhCNbLSTcHHgbbC","GGtssttVmvnnGNMQrrVzgwVrCWMz","FdhfhhcCDhHLfzclZMcrwcQMZM","HHqqCBhHSSpdmjGqmGjtjtjj","bbQLtGMQQtQRQtrDtGprrrbCqwplZhhqSqmdwvdzqqqhSmpS","FsJjJBfnsJcFcFfjVPjWBzldqhqnlZZZzzhmnSvSnm","JPcFfFWjFHJVVsVjPVscsDlLNRHGDbLRMRCDNrCGbG","JdMdlMRJnTwdvcjv","CDLHbNSzzLFgHvnTjrswBNBTNT","QgbvzSFQmZQPQQRW","NTBrNzrpjjjCwGbB","FRbQlcvFvcRQQlRsMlRRRZjwCqMwjmjwJZdLJmjCZC","cVPPQcvlWDNhrbPz","VdbVtbbZJdtJVVdDVZmTLqqTSQvNLjjDShhvSG","zplpnBnFpnrrlghGNpLNqHvqvjNj","cWncllnlPFWzcMwtWWtsVLVRmJWCds","ShLSTnZnTSttTSbLQdfSZTMwcDHwwcHnJvDHnlnlclMM","NmPMsssRrVwjDclHJwwR","gNNMWGzNmqGdtfZTbGGb","sWNNlRHnmJtmntJt","brbbBTbbFbCbqqGgBTrCfmQVVZfSSQQSVtJZSrVZ","bbFqvbDvvGGLGbCCtBGDLbLlcPNHhhccPNcdPPchlsdR","DCFvDvnCnNfMBmMMslDZML","SQQQJHwpSgJSJHQWSWHqJWWbmcBBBLLTsmhhTcZbMhmlshcb","RJRgpJHssgwSQHRqsQPGGjjtNCrrFvvnFjjPrP","mThmsgjzTPjMpcvtWP","GNNBVqVGNZbbNbNqqZQVNVNbWcpdtMCcpCtMWCdCPpQccmpp","VSmNrmmbBfZVlsrssrLTRhRhTn","TdmCvLDCpTRNTdFbbWnnSWCfhjbbzn","GrrMsPVGcQHBGMbhjjSgWfHHDbjb","BPBVqqrQPsQqwrrmmmJdRLDDqFRplT","fpDDJljDlCfDTjprjrfbddWthCSCtdPPQFhSSSWW","HsLZgMGbgBBsNzMvGbdVtVQzFRQSthhFPdtP","sMBmGBmbNvLHGMnrDppTcJmcjpqljf","ptSpSJQqpbNGGDDhcMWrlNHcZZWWls","zRLRRRjvvgjHMMsMpWpc","vmCPLCgwvwdnCzmvLbpTbVQqJJPbJPpTVq","TJCfhhJVFffrJJQQllNWcvWhwvWD","GPSGjjpLslBbpLpLqqqPDvdwvwvNzQWGzDDNdzGN","msbRjbpPqsRpHnlZrmJlnVHT","GGfFsCCTvGDsfTTrhsCMMzptZJMdpdgtrpdMcV","LBlwBHPSqjwwlVggHpnMZcVHMt","ZlZZlBbRPGGTGfmRsD","CtCjbVvzQQZTWVdd","MlSqWlmsmGBSHJHTDFHZ","pcqsmsplwsqclwRtRWgtRnPPvb","zCrzCrsdjrhGDCFqGDjRRPtpWfQQcpfQZcCZPp","VSVwVMgLHHLTwMDTMMVnbWPRZQRcRQPptWnpbZcb","MNBBBlSMvLVwTlVTFdNdhNhFsqsGDrzm","rBLWTwTThWwVVDTwHBsZZWppvpGtpptppmRvFFFMFMfL","qPPNCCbqcbcNqbqQjjJQqzjRpptmlpMGmMlJtftmtFHpMt","QnCgzzQbbQqPcPQnncbdQdnVTwDssZgrShBTVgZZsBSDHT","PFGJFqnfqmPgFJQPWdbLdpDRhbphWjDm","rclNHvcrzCNwrWRprjdMMMph","wsZHwZNvRRQsQqBV","LqlGCPlPLTCPqqQlpqLlWfBfWgcHNRJRfWNsncGH","VVtdwVtDDdVmhrdwSBmjbdzNHgfgJnNnsSnHsNffHgRsgR","wVzhbjmDbDrwjdbztFDDthMCvqPppZQBQLZQTqTvFTvZ","BnQnQFwRmRwmwdBSFDFnmSDVLCJTCTppVVmGLVTCLcgVpC","ZlWvhvZjNrbNvqjNhlfPfqjCGHrsspggTpVLpsJCpcJVgg","vPzNvqjWhqFzGSnRGMDG","wZnMZzzZZchDRtVsqtCtwV","WmWpWWmPPWrmrmBmWrTlTFPNVqVCRSDCQHcqVTtTqsSDSTSD","PrppdFlWWlfrWmpWFffrdcGjJJGggnnhZGdLLgGGndvz","FShHNmNhRhNJmBnQBQJrmP","VTgzDTjwfffwzDvwlcczzVSJbQlBQSWBWCnPJPbJWWbC","tzSVtzvSvGSRZqqFMNtpRR","hPZhGDZpnCGtDhznjmLmdJffdNzJ","glwsSrQwBvLdgLzdcj","QsRbHllzzlHwHlBszWlTBFbpDPMhbPDVGpGFpPtFPp","SRjStRDctgDSBzLvPvNrDhmPLr","QqTHGTPJmmHmhNmH","TGQZsTqFnQZCJTPsnJnZQMjVRBVtcVRSVRBlwccSCtBS","bbsNsvsvnNPTRRllbblLqhtQCqQSLCGGHSqHNC","wFpzFgqVzqVJWFDwqJDmSBBmHBHhShLQhCGSBCGH","MJVpFMqgwMqRRbZsMbZMrP","PPdDhvNDQdmgQPZmQVHHtHGGWVGbffWGvs","MMLCTRRLlLclTLRMRLCwMLHWVctbVVHWWWFfVjVGsFWW","MRSMMlpTJRqClBCRqBDnzqgQPnqgznZPZqbP","MrMNPNNpjvdprWtrpMsthqBfqlnfqcGhVBqFRcnqFG","QbDgSSQbgSDDmDVmlqSCRllRcFqnqfBl","QVJbVmwwDQbzVTgbppNJNMWNjNNPrdpM","WwJJNbtHfpLpVgZZPVFhZh","vmmqlDvRvRfqBSrlzmmMjRBhcVhQVZhVghCQQQQTcTrPTP","jSqMmqRzMDDjvqlBqsBMBmmwGNJwJnwLNfbGwddswnJtJH","RLgRmRggbvbzzPmmRNmzsQWFtSGNtwSNQnntFwnnCw","pDBrBHpHhlldphHBHhJVFSLnWWFJttCtQSttSS","hfHrpphHBppfTvmzgMmbLbgf"];

    let priorities = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let mut matches: Vec<char> = Vec::new();

    // let mut rucksacks: Vec<Rucksack> = Vec::new();

    // for s in input {
    //     rucksacks.push(make_rucksack(s));
    // }

    // for r in rucksacks {
    //     matches.push(
    //         r.first_comp
    //             .iter()
    //             .find(|&&a| r.second_comp.iter().any(|&b| b.eq(&a)))
    //             .unwrap()
    //             .clone(),
    //     );
    // }

    let groups: Vec<Vec<&str>> = input.chunks(3).map(|x| x.to_vec()).collect();

    for group in groups {
        let first = group.iter().nth(0).unwrap();
        let second = group.iter().nth(1).unwrap();
        let third = group.iter().nth(2).unwrap();

        matches.push(
            first
                .chars()
                .find(|c| second.chars().any(|a| a == *c) && third.chars().any(|b| b == *c))
                .unwrap(),
        );
    }

    let mut sum = 0;

    for c in matches {
        sum += priorities
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .position(|&n| n == c)
            .unwrap() as i32
            + 1;
    }

    println!("{}", sum);
}
