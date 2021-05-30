use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction::transfer_many,
};

pub struct Processor {}

impl Processor {
    pub fn process(_program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        // sender
        let sender_info = next_account_info(accounts_iter)?;

        // receiver
        let vec_64 = Self::pack_u64_from_u8(input);
        let vec_64 = if let Some(v) = vec_64 {
            v
        } else {
            return Err(ProgramError::InvalidInstructionData)
        };
        let vec_64_iter = &mut vec_64.into_iter();
        if accounts_iter.len() != vec_64_iter.len() {
            return Err(ProgramError::InvalidAccountData)
        }

        // zip
        let zipper: Vec<_> = accounts_iter
            .map(|a| *a.key)
            .zip(vec_64_iter)
            .collect();

        // instructions
        let ixv = transfer_many(
            &sender_info.key,
            &zipper[..],
        );

        // invoke
        for ix in ixv {
            invoke(&ix, accounts)?;
        }
        Ok(())
    }

    fn pack_u64_from_u8(buffer: &[u8]) -> Option<Vec<u64>> {
        let buffer_iter = &mut buffer.iter();
        if (buffer_iter.len() % 8) != 0 {
            return None;
        }

        let mut vec_u64: Vec<u64> = vec![];
        loop {
            let mut curr: u64 = 0;
            for _ in 0..8 {
                let buf = buffer_iter.next();
                if let Some(value) = buf {
                    curr = (curr << 8) + u64::from(*value);
                }
            }
            vec_u64.push(curr);
            if buffer_iter.len() == 0 {
                break;
            }
        }
        return Some(vec_u64);
    }
}
