use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UNAgencyState {
    pub agency_id: u32,
    pub agency_name: String,
    pub project_name: String,
    pub portfolio: String,
    pub strategy: String,
    pub target_chain: String,
    pub is_active: bool,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct MultilateralAuditRecord {
    pub audit_id: String,
    pub timestamp: u64,
    pub merkle_root_hash: String,
    pub verified_by_pqc: bool,
}

pub fn get_metadata() -> UNAgencyState {
    UNAgencyState {
        agency_id: 18,
        agency_name: "United Nations Environment Programme (UNEP)".to_string(),
        project_name: "UNEP EarthLedger AI".to_string(),
        portfolio: "Global Environmental Defense & Climate".to_string(),
        strategy: "Programmatic Utility ($UNEP) -- Article 6 Internationally Transferred Mitigation Outcomes (ITMO)".to_string(),
        target_chain: "Internet Computer (ICP) + Algorand".to_string(),
        is_active: true,
    }
}
