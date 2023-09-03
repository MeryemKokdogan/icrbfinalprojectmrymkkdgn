use candid::{CandidType, Decode, Deserialize,  Encode};
use ic_stable_structures::memory_manager::{MemoryId,   MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable, StableVec};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;

const MAX_VALUE_SIZE:u32 =100;

#[derive(CandidType, Deserialize)]
struct Proposal{
    description:String,
    approve:u32,
    reject:u32,
    pass:u32,
    is_active:bool,
    voted:Vec<candid::Principal>,
    owner: candid::Principal,
}

#[derive(CandidType, Deserialize)]

struct CreateProposal {
    description:String,
    is_active: bool,
}

impl Storable for Proposal {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Proposal{
    const MAX_SIZE:u32=MAX_VALUE_SIZE;
    const IS_FIXED:bool = false;
}


thread_local!{
    static MEMORY_MANAGER:RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new()
}