#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Address, Env, String,
    Symbol, Vec,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub completed: bool,
    pub owner: Address,
}

const TODOS: Symbol = symbol_short!("TODOS");
const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct TodoContract;

#[contractimpl]
impl TodoContract {

    // =========================
    // GET TODOS
    // =========================
    pub fn get_todos(env: Env) -> Vec<Todo> {
        env.storage()
            .instance()
            .get(&TODOS)
            .unwrap_or(Vec::new(&env))
    }

    // =========================
    // CREATE TODO
    // =========================
    pub fn create_todo(
        env: Env,
        owner: Address,
        title: String,
    ) -> String {

        owner.require_auth();

        let mut todos: Vec<Todo> = env.storage()
            .instance()
            .get(&TODOS)
            .unwrap_or(Vec::new(&env));

        // Incremental ID
        let id: u64 = env.storage()
            .instance()
            .get(&COUNTER)
            .unwrap_or(0);

        env.storage()
            .instance()
            .set(&COUNTER, &(id + 1));

        let todo = Todo {
            id,
            title,
            completed: false,
            owner,
        };

        todos.push_back(todo);

        env.storage()
            .instance()
            .set(&TODOS, &todos);

        String::from_str(&env, "Todo created")
    }

    // =========================
    // TOGGLE COMPLETE
    // =========================
    pub fn toggle_todo(
        env: Env,
        owner: Address,
        id: u64,
    ) -> String {

        owner.require_auth();

        let mut todos: Vec<Todo> = env.storage()
            .instance()
            .get(&TODOS)
            .unwrap_or(Vec::new(&env));

        for i in 0..todos.len() {

            let mut todo = todos.get(i).unwrap();

            if todo.id == id {

                if todo.owner != owner {
                    panic!("Not owner");
                }

                todo.completed = !todo.completed;

                todos.set(i, todo);

                env.storage()
                    .instance()
                    .set(&TODOS, &todos);

                return String::from_str(
                    &env,
                    "Todo updated"
                );
            }
        }

        String::from_str(&env, "Todo not found")
    }

    // =========================
    // DELETE TODO
    // =========================
    pub fn delete_todo(
        env: Env,
        owner: Address,
        id: u64,
    ) -> String {

        owner.require_auth();

        let mut todos: Vec<Todo> = env.storage()
            .instance()
            .get(&TODOS)
            .unwrap_or(Vec::new(&env));

        for i in 0..todos.len() {

            let todo = todos.get(i).unwrap();

            if todo.id == id {

                if todo.owner != owner {
                    panic!("Not owner");
                }

                todos.remove(i);

                env.storage()
                    .instance()
                    .set(&TODOS, &todos);

                return String::from_str(
                    &env,
                    "Todo deleted"
                );
            }
        }

        String::from_str(&env, "Todo not found")
    }
}