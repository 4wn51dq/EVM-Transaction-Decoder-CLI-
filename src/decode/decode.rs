

pub enum TxType {
    Legacy,
    EIP2930,
    EIP1559,
}

pub enum UnsignedTx {
    Legacy(LegacyTx),
    EIP1559(Eip1559Tx),
    EIP2930(Eip2930Tx),
}

pub struct Signature {
    pub r: u128,
    pub s: u128,
    pub v: u64,
}

pub type AccessList = Vec<AccessListItem>;

pub struct AccessListItem {
    pub address: [u8; 20],
    pub storage_keys: [u8; 32],
}

pub struct Transaction {
    pub tx_type: TxType,
    pub chain_id: Option<u32>,
    pub unsigned: UnsignedTx,
    pub signature: Signature,
}

struct LegacyTx {
    pub nonce: u64,
    pub gas_price: u128,
    pub gas_limit: u64,
    pub to: Option<[u8; 20]>, // array of 20 bytes and each element is 1 bit unsigned integer (for address)
    pub value: u128,
    pub data: Vec<u8>,
}

pub struct Eip2930Tx {
    pub nonce: u64,
    pub gas_price: u128,
    pub gas_limit: u64,
    pub to: Option<[u8; 20]>, // the None case is for contract creation.
    pub value: u128,
    pub data: Vec<u8>,
    pub access_list: AccessList,
}

pub struct Eip1559Tx {
    pub nonce: u64,
    pub max_priority_fee_per_gas: u128,
    pub max_fee_per_gas: u128,
    pub gas_limit: u64,
    pub to: Option<[u8; 20]>,
    pub value: u128,
    pub data: Vec<u8>,
    pub access_list: AccessList,
}

impl TxType {
    pub fn get_tx_type(raw: Vec<u8>) -> TxType {
    let first_byte: u8 = raw[0];
    match first_byte {
        0x01 => TxType::EIP2930,
        0x02 => TxType::EIP1559,
        _ => TxType::Legacy,
    }
}
}



