use rlp::Rlp;
pub enum TxType {
    Legacy,
    EIP2930,
    EIP1559,
}

#[derive(Debug)]
pub enum DecodeError {
    NotRlpList,
    InvalidFieldCount,
    RlpError,
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
    pub to: Option<Vec<u8>>, // array of 20 bytes and each element is 1 bit unsigned integer (for address)
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
    pub fn get_tx_type(raw: &[u8]) -> TxType {
        match raw.first() {
            // .first() returns Option<&u8>
            Some(0x01) => TxType::EIP2930,
            Some(0x02) => TxType::EIP1559,
            _ => TxType::Legacy,
        }
    }
}

fn decode_legacy(raw: &[u8]) -> Result<(LegacyTx, Signature), DecodeError> {
    let rlp = Rlp::new(raw);
    if !rlp.is_list() { 
        panic!("EVM Transaction is not an RLP List");
    }
    let field_count = rlp.item_count().unwrap();

    let nonce: u64 = rlp.val_at(0).map_err(|_| DecodeError::RlpError)?; // rlp.val_at(0).unwrap(); the .unwrap means to just trust the code
    let gas_price: u128 = rlp.val_at(1).map_err(|_| DecodeError::RlpError)?;
    let gas_limit: u64 = rlp.val_at(2).map_err(|_| DecodeError::RlpError)?;
    let to: Option<Vec<u8>> = rlp.val_at(3).map_err(|_| DecodeError::RlpError)?;
    let value: u128 = rlp.val_at(4).map_err(|_| DecodeError::RlpError)?;
    let data: Vec<u8> = rlp.val_at(5).map_err(|_| DecodeError::RlpError)?;
    let v: u64 = rlp.val_at(6).map_err(|_| DecodeError::RlpError)?;
    let s: u128 = rlp.val_at(7).map_err(|_| DecodeError::RlpError)?;
    let r: u128 = rlp.val_at(8).map_err(|_| DecodeError::RlpError)?;

    let decoded_tx = LegacyTx {
        nonce,
        gas_price,
        gas_limit,
        to,
        value,
        data,
    };

    let signature = Signature {r, s, v};

    Ok((decoded_tx, signature))
}