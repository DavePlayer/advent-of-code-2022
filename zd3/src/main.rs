use std::{collections::HashMap, ops::Index};

fn find_repeated_letter(r1: &str, r2: &str) -> Option<char> {
    let mut dictionary: HashMap<char, u16> = HashMap::new();
    for letter in r1.chars() {
        dictionary.insert(letter, *dictionary.get(&letter).unwrap_or(&0) + 1);
    }
    for letter in r2.chars() {
        match dictionary.get(&letter) {
            Some(_) => return Some(letter),
            None => {}
        }
    }

    None
}
fn main() {
    let data = "ZNNvFWHqLNPZHHqPTHHnTGBhrrpjvmwfMmpfpjBjwpmw
sbdzQgzgssgbglRtmjlwhjBlfrSrMt
zgsCRzJbsdRVQCDbcgLGWWLnZNGVLLZMNZnq
tvHhRtZGMvMHvfsrBBCTRbwbccRc
qznnlpzzDppWlDpQpCrcrwnBNwTZnBTZrn
PdVZJJqVZdllDPFtMjMgLjGMHvSgMF
csbhhVDDvzlVDcbccGGvfRjDHCjNLRHRCLfmnZfR
dFrStSTTmrrrHVfV
MMgQMMTMVTdgWtwTPwSgWSgGbbppJzlplvhBlPbzhlhbzG
FDJSTtSGhpPFDmFTZDpTFPmCBBrHqsCBhgBlqqrqrlRrHH
dQwMtfdzVwWfwctwnfnQCHllzRrsNzrrgNlCgqsr
fLfQnVjfwQfMdfvfnVvWDvtJPFGDpvZGbZpmbSPP
TzzCrJcDrTDdLDCJDvGNPCFqlZWlvNvWpq
RRHfjsQBFsjgjBQsWqGpNvZQqQlPPQPN
VnHBnRVssnnjsSfBwbMSrrbTwJTcwSDF
HJCgHCCFFFVGJWTlbqDdlqTDDpgl
cZccSmLrfZcrmmzSQftdpDtTHdbQTDMQ
NZZccrrBwZRPNNzmcLSSjJhGhVWCnsFnHBjGChsJ
qwwwJHTHqdFDtZBFPfFBZFzM
gVRcLnnWVgggnnnQgVWWNZtZrBfLBzZzBrMPPrZvPv
GQgQSVRtsVnNRGSCdpmwspmbmDpHmhwd
bhNgNfgwpbLMhCZMGQBmDm
FrcHrSllcqcFFMGLBDQlMDTGlT
FVSddRSJRjLwbjJPJw
wzhhrTwwTrSsdHQjjSHnBjQj
gRDCmVgRgMvtMfVMRBBBhWCHQQHGJHZJQZ
NtgVgttVbMNmvsNlpcrLhLTNPw
MCgjsfnscgjjgnGgJHHqHDgdHbGr
QSSmRFPpRtPFQLQRmPzvBzzzDWqrqWWHJGGNrJJbdtVWHJDV
BdSFdLQzRFlSLmQplffwncfscChhcsMj
GfVmfnmJVnNVFhnhGmbmhpHvqjrzHZBjfvrtBHHZrwBt
ddWQldlMdWMlQsLWTLQgMNwBrvjrZjNrwzZjswHqrv
QQdTRcgTRPDlMQlQPQdhcNNnbJmbGpVnGchFmm
CjjZCCZfvWZRHHhRtwhvPN
mrnqlqMqBlSSLnBTLBwmHPPWhPPHtFRPWzwt
rBVTrrMMSMLQBrndGcddWQbbdZfCZJ
LFtdjHjLjLqHqstLTjFLFqNMnMhhZdDDNMVbWdDDbhnZ
CrBpBGnzrzmczcllrphCZZWJMDWRbbZNMDMR
GwgvzpzvrcmBrnfHjTgqTsgHjF
rMPPZcplCZlZPwtSwhtBwCQQzB
FvDGffLqqmQFwmmhzt
TjJjJfHHVDVnHVgZZlQppcVscP
hVcqHwhgwwwjHjjGWbvrbBGrsWVWGn
CttPRpMmPDTWbWltlLBnGl
pZmDFMmPMfnZwqqwfcqJdHgz
bSJWhWJCbGGWJPStWTgRQwzDjgQQjsDW
nFBBVQVrVBrNFMFZVpBBZFZrDgdTldgsRsslsljsRzTRjzns
rMcZcHcBQPvbbHGP
mSfmwqfmzrfHwFfmrwvPHqPmMFRlMDDZBCVVRCVZVlZpMRRR
TWjdTWhTsssLTGsJNWhTQddjRMDMtNNBSCDBllMMBVtDMVRZ
QhWTQcdhjThsdGbTLGjWHmffnmHwnwHrwqmmfcwS
LmrsMQnnpfmMLllvTvqvFFzvFHNN
WGRFVWdwZWZvCbJzcvJNzw
VjGhDtWGSFRGjVVSFdjjDPBfspPnnMBLPLrrpMMm
qqqCCJjtqtqCtqLZspHWBdSrWWSzzbzHFWBldb
GhwwcwPFVDcNFRRGwwzmlBrBWvllrvSzlrcd
DGGhQNNDhTpZZqqLQFQQ
QfZmgQQZCCMLfNrgprdNvvdrTg
hhttsBmBDcFRBlJshJcRrnjnTvNqpddNNqvndp
JtsGJGtGGJJJHDbctllhZHmMwMQSPVPzHSLMPZmV
DScSjZcNBZqjDDcLLfFtPfCfjfPvfv
pTmRlWhdMwTLGwCf
mRdWCVVglWrCmVHVrVCmdrbSzNcBDBqBZDNHqssscNzqNc
sPMHGFMsrPNCPnNS
ffJzllbzpZBllttBtfglgBTbSCVCmmrNFmmbFNvCFLLb
cpZqpfgZZJtJqJJJfWHWhHdHWHjcdRdFHD
ZZPfppvzMrlNBFcvFB
shJgstJwWLVJwcrFFVFrBVNNqFFB
HwWJdLHWWLcQgssHwwSQSQtQzCnZZMpZCmdzZCzpPzpCPRCj
QCpLRbsCCQQLbQzCBQDQBBfTTffWtTctJVRNVtnfwtWV
GvlqqlGlmMrdsvrhmlcTvwJtwNwTvfJfcWTW
lMhgqGhddjqFFCzBBpbsSQpD
JJwGJwVQQwVSsSMhQMQgHfgfTtrrfVTNgNNfrt
dFDWCDdFppvDFmWWWnJTPllHmHlgrqrgggtH
DzFbWjdRpbdFCjjRbnFbQBGhhQBBJZwMhScwZwJz
HttvHpHmpJWtHmFNvlvdMSVdPMtLVCCMMMfcfL
GjgzhGSGSSdCcRMVjMdc
QshbnghgnGDnqsFrNSJFrsNs
wJpjMwzjzdVbzPPVpbCHnqGnBqnsBrNCwgrC
ftTLLDTQtLTGTGtFrgHrvqgQnrvQsCHH
fTcFFfLSfFFcGFllcFhPJPjWWJSjSWzMWPdS
ZjNdmjVQVZmvNNZNNZHWZmWtsJnwTpJJswpWwGqJhJqGpp
FcRRcDblDMLRcRMLFFMDGsJnqhwpqTTJGwnsfnlp
LRBrcLbbgLFgBbFqDvdHQvCCjNzzzVrZdV
BdbLWrgdvgWvVJgWnDfNhVnqhCCpDpcq
tSQPSTSGPMmlMPtQQPJGtGQRCcnqqfnRhCcChDqnCfRScf
jTssPsjMQMmszPjlTtsJdFBFrJzrbJdHZFHdWH
vCccctvvTTtZcgLGcZTbssbMWnpMpmLWqnNjpfPPfPjMPp
wwBBlRBBwDDVFRhFlRhdRRVWPnnpMpffmmffrpWqVNPm
ddhddRzHlQHFJcGsCztTgbNzST
fJctfpVWcnfRLfrRwP
vmmnvDQDZTNTmGGTqTMTvMqwBdLjBvRzBRrRBRLjjBPzBB
GMnmqSTFFQqttcbcJWgsSt
rHNfmfRsmfRGfDNcRmcmMQlLCGSnQwwPPCSnzQlSCl
bsJTBsVhFsVpqFWFgPCwnQwBZzwQzZLlzn
qggTTqvqgqbbTTFqVqgWqvNmmMMRdffftNfMDMmscR
rFWQFszrwjsjFWvshPTCmLZLSTLwSLlgSP
BQbcqVHNVqVpVpmClJgJJHSmZLJm
qBNNNVdDMGBpDcDWsvdQsFrFnjttfj
qGhmttmzhtMvhbrLdSHbdSHRzb
WCBgQJJpjCQlgdHZrfPRPSRbNg
jBTTDjlnjnJDJTQCVntcwtwMSvqcGFDhcvsh
ZTrnTqMWWWnfrddMGJPgPLlPbw
VvmGRVpBpNNmvNvjVjtpNpCNLLLJHHBdgLPdwsdsbLlwwlwb
GmCVSCRVGmpCRVvttmpDrQZfhnzhzqnDWnrZZTQq
DQBZHHtWHzSvZvDQWchgqsqqhrrhhcqrcZ
jdMfwlFfFlTfndwpjjwGnNrqhPTmPSPTPPhmgrPSrh
jlGbwGMdlnJpGFGjpnFCSJzzDDtWHCBBQBvtVC
RrbBWBRRWSRsBBVvsPHZDwSjjPdnHwtPtH
fTgfzMmNJpmJgfllgpjVQtDDndVQpdnHVtPp
gGmlNclTGmGFhLVcVrvLqrvc
QcpCTVCZVcCwLcCVvHvvVsCcNzNNSbPRzsDRDSBlsNNzDRtb
fggMfJqgrWFpmjWMggmrfMWNSbRSPBDbNtJRtPJzlStBbN
gdnmpWGnZvdQCvdv
tqqcLqqDDqNtDrqHrrPWlTlTWZTMzTFzQlMPSZ
pfnpmmppmppRGjwbjmnjwspWbQQQTMWZbCTSZCSQlCllZF
gmpVnGmmmpjDvVLBFqqvrH
LqBvJHZvbHGBHrBtGGQTmSVprVzhpVPDPQzQ
CRdRgwCfhTVDzSdQ
fRCcjgSMjfNgMMLGbGZtvBbGHv
HgvtDDzDpvwgvvqdHPZWdMssTTddSs
rJFrGNFVQmNFVmRnWhhsrTbhwhZTrdTd
VQGBBBVNQClpcBvBwD
PWlSzZGmdmGmlGmhggBpvMjvMjFgPJ
TtLRDtQQfTVcQQQRtBsJFFccFjWhJJFMBs
HqVCNtWHCDwdnlGwGqSr
RwdRJgCJRGGmdMbcGbdnTnTtttLLnptMtMtMqZ
DWsWPFrPqVPPLVCB
zQWWsslsQHFhDSszDSFQzJJJmvcgblRgmNvCJmvNgw
tpmFrWTtRpRTtggsSlnQpsnnlSHPsn
bZwZjNNZGLSrVsGndPPV
NvrcjCfbvvLBDBWfWFgRRm
WWFMgWmMhhwDcMMMDcmLWLtQwwsjbsQHvZHbRjZfsZzH
PTCplTCdSJJCpvPGNSvsbsfHtbQZzdHjQtjjsj
vNGJPpqJvJvqghgFgWFmLD
RlRpLTZCjWRjRWwpRsjHjbSbqMqMvvnbnGMnGGqQCq
gddfDNczmgPthNcDdgPVnbbzbnJrJJGSSVJJQS
BmDmcDmcmhffdBHlRwjRLpwlWQ
prQlfzlWRPzgQWzlMPMRppssHHsDsHjwnHHbWDwwbwjL
vFBJJtZNShJvZFtdSqtmqjTDVHVGDHbwVHDVsDnThH
vcjBZZdZqvCfpzRfcgRp
cggpqgRlSpNsgNggbjjj
ZZSSJVLVLFDZWNGjCWWbCjsF
vZLvfZQQfQtJVJDQShLrLfMmnldmwqwTqqMcMTMTndrm
bQBMtBPddtMFbJFhRGzMfzvnRGRSvWnW
TmHTqlVHwVpQqjmwGvSgSpnLpzfWGWSn
TTrDQCDrrTmDCCCVHHQZBdZFPdsNdFBtFDhtFB
fjpQvNZcGhGGTtQS
DVJzvbVmHbbtSTSTRStzTM
VDvmqllmJfjWlnplNs
ZmdHZJjvQLdRjpmLJrqqZBhhtCschPfBPcrDfPffCD
MWWSMMwnwlSgzWFFgSwzVwzqcfDCfChCbbtssbfDChcD
NMqFTwGqMwgwwgjHRdHRjdmQmQTm
TTqWPCWRhTWqPNjPJMNtrlbJFttQwwrBrlbwlc
GfpSDGZvpQffSHDgggDZrHctFmrHncnnwwbBtBrt
SQGfLsSLZsqMTRNMPT
HdBdnBZJTZBBmsfwwBlh
MjCVjzwqWrfzplzW
vVbqCjjRgjwMbnbGHJScScZHLL
dwwwtCdznvDDFrMrrw
GmWLQmgQmHgcdGcsTgTDqDbSfFWfMDMfbSNqvr
QhTLmVQHLmdLTjGGVptRnZpZBZVRpPpP
CzjFpzRHdtBFBCqNqSbJZWcQJTSbQjMTWZ
wGwVLlGrdVGwDnwsgfMSZvJMbWJcWlvbbMSc
rDfsgggrGnGngsPwdVLfDnmDtzzFNCPHtzCtFHpBRqhPztzR
mrgWzBcDtVCcQcCCdscf
LRJhjRjPZvqSRGhGjLgMCdHpMNwQCpMHpHMS
GRvGJRJjqPZbvGGhRjnqLJWtgFgtzTzDrFnTWrlTlllW
cbmcddlffvbTfvFflpZzsMVNznNVlnqnzqHMNM
StWJBQRWLRWNPNMCswRVHC
BJQBhSWhjSthJQGGWWggJDDDfbdbbfHbddbrFrddvFvv
jFqvqvWZWDtBJrrlrq
TzGcbHcrmVzMGNSmTcGDtBthJCNtsJDlBCghgP
bTrnTccnLSrrTHbnwfLjfdvRRwZFdwfR
drHVrdVDfsDbVsdVDbVqRwbZZwCRCCCJlJThwRgT
jFPcFpBSvtNPzSFcjcQpcQjpThZCRltGRRRJhwCwGhwgwhRm
SQSzPBjjPPSvLqqssdnqLZLMsM
bQTWlWlvQclNwwWlCCLStCRSSjStpj
zVZZDdBnBmgzVsjsLthSpshdCL
DfBnrmBmgzHBfDHmnGrNFCwQvTPvqCTwqTFGbF
srSWJnrbmlWlbhzsWszSvPGwvgDhcjdjjfvhjvGv
BRRQFLtNfQNMpqpQHDjdDjDcZZcvwZZHPH
NLCNCtRQfRttRFRCTqMBqQQrzrbzrlJmVVbsSWmVrTbSzJ
RHLfLcSRTFSghLRHGbwZmMZddgJswZsbMm
ptqjtCzzQztqCjDlBGpDpbMZdwmMbZsdwNmdJpbs
tttzCVllDCtDQnQBVHGHWvWTLWcLSLHf
FVlNnPqbGTHftghggJqf
zLcZWZpWWrcrZLLZDWrwMcrhBFBttChBmBgptChhtFftmf
LZZLrDrrDDMrcwrDwsWFzdTlnGQPQQVbdbnsvnvsVQ
BbPNMJNbQvDbvPLwHflczlwwzf
pZjWZGZjFGdgpnVgZhghdmcflrlswzzcstlrLwhtwc
WZSdqFjqSqSWdGFjZpdMTTDNTvLCRRLLqRQMCN
FqgFGtbgTvRwrLqhvw
JCCWJWCdJMQNNsSWsMPQRDDLDSDLwTrrvnwfDvnD
HdPJlBBHCCQdBMWdTtVbgHczGVGjmtzG
PLlZDLZDsFCvbDQv
HVcTmVmJqVzqczfzbjvvCFMRfCsWjMvR
cqHzTqJTTTTzzmnmrctrBlLlvSlgLdZvSwSlpw
SbMMNJjmgMnJdSSbjVFZVSQrlQfWVQVWZh
PtqDqPGcLHzHpqLcRzRsfQFfZlfRfZfRFVsl
cTDLcqGCzDTqzzDLDzqPTtJvbBJMnmvjbdlmJNvmdgNC
tDJDlZVqJGbvHNQbNFFsFPmLns
ppczpzpffGwfBNLGmn
WShzgTTpWzhWztJJGJSvtvvtjq
TbZFTFScnCZFQRTCqQdBjdJqjBqjjQDB
rmmLpLLfzrlmslMBHvdRddNDDJDrqD
MWwLPzmWfpsMmmlMPMWLwRTZTZnnTcVCcZFCwSnZ
SqmClqHssNWCqPTcWcGhBTchVV
ZnnnDflRpBVTTVhPBZ
DpgfvnvMfCsqlMtSll
ZzLMRZpLMwwppZqnQGvQgBSvlNVlBFFNFVrg
HcqhTmhmdDTPFTJgTTFBSgJN
mccPdDDHbssbtwZMqpbzCRGM
TgqnTltgWqLRSRnlqddngFfrvHvrBTfCCFrFVTvVCf
cwNJmPzQwNzczzNsJGhhHfhrfvVHGvtvVVfC
jjtbtDswcmPWlbgRnRdMZL
TmpTBBwvspTptRmsmTGLQDGRHGgVGLSQSMHQ
ZlPWqjWrzjPqdrlzbrbrwfrWLHVMLnHDMVDQnLQfQfVngQLS
zNwbrrFWbFJpmpmvvt
RMQQMwHMMzcFsWsDrWfcpJpS
LLhZmGVLhVlTZfWWfWpCrDsGSp
VLVTnqjjZngtQRFjvzDM
gmRBpjrpRvCfRCrBgvjHShnbnngbgSJnNsHMHS
ZDPTwGWtqwHhSnbcMNJw
DWGGqtVVqldWZzMzWmvjrjprLRFjRVvvff
tCzVzsVtDFzssnSsgdqJdCNqJhmgmpqq
PZccPGvQfRLMQwNdhpwhNh
jLrcbRjPZBrcPdjRHFlWnVtBFslSWznW
vvvbJbWrLvFWHzZzZRhB
chtwTmCNlRRZzRPT
hmcCssCswrMDGMSrsr
LStGBsQLlllhzMzs
dzVZDNWRDdZNDTZTPvWVhhphpMlfMccRmfnlMlRn
VFvgTrNPdFWNNFNFTzTFFSjSQBCqrtQwSBGLLBGwGL
qGJSJhWStdSfWvSvtGRRnzRDDggrgvnzsmRP
lTTLpcljjGlLlLNBpjwFQDQmRnrRDPrPscRrDDng
NCNjFlHNCTVjpwGqGSVbJddqZZJM
MbWdgvHFlMvmzTzShvmm
tqjqpLsNsrrsjstNLpQrGVhVBzrhVcfmchDcTPVVmc
RqwjqjqsGjjGGQNjGpQZpqRFJgmMHwdbFWgnHMFdwmmCFW
HHHLcCcVHjTHglsB
wDSRwzzRpMSdNSPSwSpRbqvgBsdqlgTvBFBjgFvvgB
RpbzPssDMWwNRbRNRPDsDhJthLQVGLJcctQCJQfQJCLm
WsZgbNgZVCCWbVVVmgZbCCRPccGnzPBqJjzWJBJPzvBvGz
SpfThHtrHFBPPzJvPntj
QHDhhrhpTQpHhQHnfwnTCNlbZCCDLNllZlVsNCNl
QtzJFRQLMRnZcZsfcphlPQ
qSBbjmWSCNmVldSqqSqmjCSZshfwfrPPZZfcPVZfhgsgPg
HqBbHqBGSlNBbltnLLHFJMtRvRTD
tcGtDdMcttttHNBlMctldlwjwwqqCLCwDwZjFCZhmnwC
VrJgvWWsPvRgVgrJQvfQfzgVzZwCbLZmnmwCwZqmnhjZbnLj
sJpffsRWWRJVWWpHltSpnMHGcMTl
zNqRbqSbfdcTLLfS
ZVPzPnVvdLwLDPfF
VWnzQCVWZVMzQRHgqgqrHGtGMp
PbHpWfWPvRfbzWPFfRpPDtBwSHMwCBgDwBjDtMMM
hTTdZQlcnTcmqVTdcddrDgBSwsjjBgqBtsCgMD
hlldTmdJJmJdZvzfFfNJFJgRzR
PJWvJBbWsfLQWsLvmCqHCcNLHqHLLcwDqV
dQztrZrdwHhptqDH
ZrMGjgMSrdzQGQRJPvGGbm
RmjljZChlDZBCRRvlmNSLSqMNLzwLvppwQSQ
sTnVnPrVGsGTPddJrfgQgqLgGpMNQtgNtNzg
sbbTfTdcJPnHbsJfHsdcmDDmmqBZlClmjBRDCZ
CJmHLmHFFCFbHsbJsJqvqhQqLDhQZvnQDZnn
wGwppTjdWPdgFpGcScBqNnNqNhQlDqnDlZZW
pGcgGgTpGjFdwpSFVgSdpPjrMCMffzJzRzztRfHCRsVmtbsz
CgBClZfCflPflNZRvfQswwmwmwQsQhgppdhm
qbzDGrjLLNLDHDqtJmmhhmQdhwpQhhbp
NLGqVqjDjjGrMFrvFWPBRBZnCvfFnT
tbrrHsgsVmmmbtgwVsQRqjJMmqMjQfJfLFLD
ZvlBGzdvjGfRFJQJ
dBppnnBBhdzZncBPlznpnNdWHSsbWthbSCgHrVfgSSwVgr
VRvMtRVFHQLvMRQFQtBctrthshTTgCmhTrgWhWZsZZ
lzJlGBSPPhzjgZsTCr
wJlpJPfDSpwBnddqJDdpPpcvMFHFMvNbvnNMFHHRVVbR
CPShbbdlGCdQqlRPGPdlDWDFzjtFjggCDJgWczfF
mrHrTrrBMBsmNsrwsBpnfpggDDcjjDDpjzFJzzjtJz
BvsNvBLHrrrNvwBTNNsNGbdQhlPGGfqhhRGqLGdl
PSSlPtlStGhPNMtwPMPJzDddnbnDNTDDnJqjbz
FFVHRwVLvFvVrVHrZcLmRHggjDmdDnDnznnznzQjzdmJddbn
WrvgRgcRcRrrcRvgcVrHVrwCCSfsCsGsllhMSSSSMttlSCpG
hBPJqVZTqqPSlGlfddfddZvl
JWWMJCpnMrmztzdjnzld
RbWsrwMrpbRspbWgpwhLJPccNVqLLPSVgVPV
hcTrWqcfhwGfWrWMjHjGvDHPmJMDzF
ZtlsnZZtLBSbSssnbndjDJJFHFHJPHPsHMTHHM
ntRZtSbtZgZStTqchwQfRwNpcq
GfLqrsqQGgPgjjQGVcNvTpTpNFcWPvPPpT
bRnRLnMZFdCMcpvT
RnRhzRlmlhhHhhmhRsqLrfzrGVSrGBSGrL
fbMffwdZsncrGcfG
qDBjSSLqhLBSmDbjqNhqTLjCGrCHGrvcGWcpWcrGWnCrpm
STLDqbhTLqNTNSRhlwZlJlRQFFRwMdPQ
TVVGNFggcjPPJzwvQlRRwRvSlcSc
frsBbWhtSRzSLfRf
qDCqddbsWrqzhsdNmdJNJHjTggFFVV
NTWTDrSdFTLtPTGf
lZqjHlVRvRltLtRWFMtFLL
qvjWzzvVbZpjqllggscdchwDrCphwsdhrD";

    let example_data = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    // zad1(data);
    zad2(data);
    zad2(example_data);
}

fn zad1(data: &str) {
    let mut item_sums = 0;
    for line in data.lines() {
        let (rucksack1, rucksack2) = line.split_at(line.len() / 2);
        let repeted_letter = find_repeated_letter(rucksack1, rucksack2).unwrap();
        let alph = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let repeted_letter_priority = alph.chars().position(|l| l == repeted_letter).unwrap();
        item_sums += repeted_letter_priority + 1;
    }
    println!("sum: {}", item_sums);
}
fn zad2(zad2: &str) {
    let groups_rucksacks = zad2.split("\n").collect::<Vec<&str>>();
    let mut item_sums = 0;
    for i in (0..groups_rucksacks.len()).step_by(3) {
        let rucksacks = (
            groups_rucksacks[i],
            groups_rucksacks[i + 1],
            groups_rucksacks[i + 2],
        );
        let group_type = find_group_type(rucksacks.0, rucksacks.1, rucksacks.2).unwrap();
        let alph = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let group_priority = alph.chars().position(|l| l == group_type).unwrap();
        // println!(
        //     "{} -> {}  --- {}",
        //     group_type,
        //     group_priority + 1,
        //     item_sums
        // );
        item_sums += group_priority + 1;
    }
    println!("sum: {}", item_sums);
}

fn find_group_type(r1: &str, r2: &str, r3: &str) -> Option<char> {
    let mut dictionary: HashMap<char, u16> = HashMap::new();
    for letter in r1.chars() {
        dictionary.insert(letter, *dictionary.get(&letter).unwrap_or(&1));
    }
    for letter in r2.chars() {
        match dictionary.get(&letter) {
            Some(_) => {
                dictionary.insert(letter, 2);
            }
            None => {}
        };
    }
    for letter in r3.chars() {
        match dictionary.get(&letter) {
            Some(num) => {
                if num >= &2 {
                    return Some(letter);
                }
            }
            _ => {}
        }
    }

    None
}
