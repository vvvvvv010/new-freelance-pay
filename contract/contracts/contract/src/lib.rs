#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, Symbol, Address, Map, Vec, log
};

// ====== DATA STRUCTURES ======

#[contracttype]
#[derive(Clone)]
pub struct Session {
    pub user: Address,
    pub expires_at: u64,
}

// ====== STORAGE KEYS ======

const SESSIONS: Symbol = Symbol::short("SESS");

// ====== CONTRACT ======

#[contract]
pub struct NetLeaseContract;

#[contractimpl]
impl NetLeaseContract {

    // ====== START SESSION ======
    // User gọi hàm này sau khi thanh toán (frontend kiểm soát amount)
    pub fn start_session(env: Env, user: Address, duration_secs: u64) {
        user.require_auth();

        let now = env.ledger().timestamp();

        let session = Session {
            user: user.clone(),
            expires_at: now + duration_secs,
        };

        let mut sessions: Map<Address, Session> =
            env.storage().instance().get(&SESSIONS).unwrap_or(Map::new(&env));

        sessions.set(user.clone(), session);

        env.storage().instance().set(&SESSIONS, &sessions);

        log!(&env, "Session started for user");
    }

    // ====== CHECK SESSION ======
    pub fn is_active(env: Env, user: Address) -> bool {
        let sessions: Map<Address, Session> =
            env.storage().instance().get(&SESSIONS).unwrap_or(Map::new(&env));

        if let Some(session) = sessions.get(user) {
            let now = env.ledger().timestamp();
            return now < session.expires_at;
        }

        false
    }

    // ====== GET SESSION INFO ======
    pub fn get_session(env: Env, user: Address) -> Option<Session> {
        let sessions: Map<Address, Session> =
            env.storage().instance().get(&SESSIONS).unwrap_or(Map::new(&env));

        sessions.get(user)
    }

    // ====== END SESSION (OPTIONAL) ======
    pub fn end_session(env: Env, user: Address) {
        user.require_auth();

        let mut sessions: Map<Address, Session> =
            env.storage().instance().get(&SESSIONS).unwrap_or(Map::new(&env));

        sessions.remove(user.clone());

        env.storage().instance().set(&SESSIONS, &sessions);

        log!(&env, "Session ended");
    }
}