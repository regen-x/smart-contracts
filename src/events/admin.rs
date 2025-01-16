use soroban_sdk::{Address, Env, Symbol, vec};

pub(crate) fn admin_changed(env: &Env, old_admin: &Address, new_admin: &Address) {
    let data = vec![&env, old_admin.clone(), new_admin.clone()];
    let topics = (Symbol::new(env, "admin_changed"), old_admin);

    env.events().publish(topics, data);
}
