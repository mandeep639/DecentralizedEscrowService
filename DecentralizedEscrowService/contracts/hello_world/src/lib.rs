#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, symbol_short};

/// Status of the escrow
#[contracttype]
#[derive(Clone)]
pub struct EscrowStatus {
    pub amount: u64,      // Amount of Lumens (XLM) held in escrow
    pub is_active: bool,  // If the escrow is active
    pub released: bool,   // If the funds have been released
}

/// Mapping escrow ID to its respective status
#[contracttype]
pub enum EscrowBook { 
    Escrow(u64)
}

const ESCROW_COUNT: Symbol = symbol_short!("ESC_COUNT");

/// Escrow structure with fields for storing details
#[contracttype]
#[derive(Clone)] 
pub struct Escrow {
    pub escrow_id: u64,
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub is_active: bool,
    pub released: bool,
    pub release_condition: String,
}

#[contract]
pub struct DecentralizedEscrowContract;

#[contractimpl]
impl DecentralizedEscrowContract {

    /// Creates a new escrow
    pub fn create_escrow(env: Env, sender: String, receiver: String, amount: u64, release_condition: String) -> u64 {
        let mut escrow_count: u64 = env.storage().instance().get(&ESCROW_COUNT).unwrap_or(0);
        escrow_count += 1;

        let escrow = Escrow {
            escrow_id: escrow_count,
            sender,
            receiver,
            amount,
            is_active: true,
            released: false,
            release_condition,
        };

        let status = EscrowStatus {
            amount,
            is_active: true,
            released: false,
        };

        env.storage().instance().set(&EscrowBook::Escrow(escrow_count), &escrow);
        env.storage().instance().set(&ESCROW_COUNT, &escrow_count);

        log!(&env, "Escrow created with ID: {}", escrow_count);
        escrow_count
    }

    /// Releases funds from escrow if conditions are met
    pub fn release_funds(env: Env, escrow_id: u64, condition_met: bool) {
        let mut escrow = Self::view_escrow(env.clone(), escrow_id);
        let mut status = Self::view_escrow_status(env.clone(), escrow_id);

        if escrow.is_active && !escrow.released && condition_met {
            escrow.released = true;
            status.released = true;

            // Simulate releasing funds by updating the escrow status
            env.storage().instance().set(&EscrowBook::Escrow(escrow_id), &escrow);
            env.storage().instance().set(&EscrowBook::Escrow(escrow_id), &status);

            log!(&env, "Funds released for Escrow ID: {}", escrow_id);
        } else {
            log!(&env, "Cannot release funds. Escrow is either inactive or funds have already been released.");
            panic!("Cannot release funds.");
        }
    }

    /// Cancels an escrow if not yet released
    pub fn cancel_escrow(env: Env, escrow_id: u64) {
        let mut escrow = Self::view_escrow(env.clone(), escrow_id);
        let mut status = Self::view_escrow_status(env.clone(), escrow_id);

        if escrow.is_active && !escrow.released {
            escrow.is_active = false;
            status.is_active = false;

            // Simulate canceling escrow by updating the status
            env.storage().instance().set(&EscrowBook::Escrow(escrow_id), &escrow);
            env.storage().instance().set(&EscrowBook::Escrow(escrow_id), &status);

            log!(&env, "Escrow ID: {} has been canceled", escrow_id);
        } else {
            log!(&env, "Cannot cancel escrow. It may already be inactive or funds released.");
            panic!("Cannot cancel escrow.");
        }
    }

    /// Views the status of an escrow
    pub fn view_escrow_status(env: Env, escrow_id: u64) -> EscrowStatus {
        env.storage().instance().get(&EscrowBook::Escrow(escrow_id)).unwrap_or(EscrowStatus {
            amount: 0,
            is_active: false,
            released: false,
        })
    }

    /// Views the details of an escrow
    pub fn view_escrow(env: Env, escrow_id: u64) -> Escrow {
        env.storage().instance().get(&EscrowBook::Escrow(escrow_id)).unwrap_or(Escrow {
            escrow_id: 0,
            sender: String::from_str(&env, "Not_Found"),
            receiver: String::from_str(&env, "Not_Found"),
            amount: 0,
            is_active: false,
            released: false,
            release_condition: String::from_str(&env, "None"),
        })
    }
}
