#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec,
};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Expense {
    id: u64,
    item_name: String,
    amount: u64,
    category: String,
    note: String,
}

const EXPENSE_DATA: Symbol = symbol_short!("EXPENSE");

#[contract]
pub struct PinkPocketContract;

#[contractimpl]
impl PinkPocketContract {
    pub fn get_expenses(env: Env) -> Vec<Expense> {
        env.storage()
            .instance()
            .get(&EXPENSE_DATA)
            .unwrap_or(Vec::new(&env))
    }

    pub fn create_expense(
        env: Env,
        item_name: String,
        amount: u64,
        category: String,
        note: String,
    ) -> String {
        let mut expenses: Vec<Expense> = env
            .storage()
            .instance()
            .get(&EXPENSE_DATA)
            .unwrap_or(Vec::new(&env));

        let expense = Expense {
            id: env.prng().gen::<u64>(),
            item_name,
            amount,
            category,
            note,
        };

        expenses.push_back(expense);

        env.storage().instance().set(&EXPENSE_DATA, &expenses);

        String::from_str(&env, "Expense successfully added")
    }

    pub fn delete_expense(env: Env, id: u64) -> String {
        let mut expenses: Vec<Expense> = env
            .storage()
            .instance()
            .get(&EXPENSE_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..expenses.len() {
            if expenses.get(i).unwrap().id == id {
                expenses.remove(i);

                env.storage().instance().set(&EXPENSE_DATA, &expenses);

                return String::from_str(&env, "Expense successfully deleted");
            }
        }

        String::from_str(&env, "Expense not found")
    }

    pub fn update_expense_note(
        env: Env,
        id: u64,
        new_note: String,
    ) -> String {
        let mut expenses: Vec<Expense> = env
            .storage()
            .instance()
            .get(&EXPENSE_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..expenses.len() {
            let mut expense = expenses.get(i).unwrap();

            if expense.id == id {
                expense.note = new_note;

                expenses.set(i, expense);

                env.storage().instance().set(&EXPENSE_DATA, &expenses);

                return String::from_str(&env, "Expense note successfully updated");
            }
        }

        String::from_str(&env, "Expense not found")
    }

    pub fn get_total_expense(env: Env) -> u64 {
        let expenses: Vec<Expense> = env
            .storage()
            .instance()
            .get(&EXPENSE_DATA)
            .unwrap_or(Vec::new(&env));

        let mut total: u64 = 0;

        for i in 0..expenses.len() {
            total += expenses.get(i).unwrap().amount;
        }

        total
    }

    pub fn get_expense_count(env: Env) -> u32 {
        let expenses: Vec<Expense> = env
            .storage()
            .instance()
            .get(&EXPENSE_DATA)
            .unwrap_or(Vec::new(&env));

        expenses.len()
    }

    pub fn get_financial_summary(env: Env) -> String {
        let total = Self::get_total_expense(env.clone());

        if total == 0 {
            return String::from_str(&env, "No expenses recorded yet");
        }

        if total < 100_000 {
            return String::from_str(&env, "Low spending activity");
        }

        if total < 500_000 {
            return String::from_str(&env, "Moderate spending activity");
        }

        String::from_str(&env, "High spending activity")
    }
}

mod test;