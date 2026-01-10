use borsh::BorshDeserialize;
use solana_account_info::AccountInfo;
use solana_msg::msg;
use solana_program_error::ProgramResult;
use solana_pubkey::Pubkey;

use crate::instruction::ClmmInstruction;
use crate::instructions::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = ClmmInstruction::try_from_slice(instruction_data)
        .map_err(|_| solana_program_error::INVALID_INSTRUCTION_DATA)?;

    match instruction {
        ClmmInstruction::InitializeConfig {
            index,
            trade_fee_rate,
            protocol_fee_rate,
            tick_spacing,
        } => {
            msg!("Instruction: InitializeConfig");

            Ok(())
        }
        ClmmInstruction::CreatePool { sqrt_price_x64 } => {
            msg!("Instruction: CreatePool");
            // TODO: Implement
            Ok(())
        }
        ClmmInstruction::OpenPosition {
            tick_lower,
            tick_upper,
            liquidity,
            amount_0_max,
            amount_1_max,
        } => {
            msg!("Instruction: OpenPosition");
            Ok(())
        }
        ClmmInstruction::IncreaseLiquidity {
            liquidity,
            amount_0_max,
            amount_1_max,
        } => {
            msg!("Instruction: IncreaseLiquidity");
            Ok(())
        }
        ClmmInstruction::DecreaseLiquidity {
            liquidity,
            amount_0_min,
            amount_1_min,
        } => {
            msg!("Instruction: DecreaseLiquidity");

            Ok(())
        }
        ClmmInstruction::Swap {
            amount_in,
            minimum_amount_out,
            sqrt_price_limit_x64,
            is_base_input,
        } => {
            msg!("Instruction: Swap");
            Ok(())
        }
        ClmmInstruction::CollectFees => {
            msg!("Instruction: CollectFees");
            Ok(())
        }
    }
}
