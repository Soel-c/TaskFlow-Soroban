#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, vec, Env, String, Vec, symbol_short, Symbol};

// 1. Definisi struktur data untuk Task [cite: 318, 319, 320]
#[contracttype]
#[derive(Clone, Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub is_completed: bool,
}

// 2. Mendefinisikan Key untuk penyimpanan di Blockchain [cite: 366, 378]
const TASK_DATA: Symbol = symbol_short!("TASKS");

#[contract]
pub struct TaskContract;

#[contractimpl]
impl TaskContract {
    
    // FUNGSI CREATE: Menambah tugas baru [cite: 341, 347, 349]
    pub fn add_task(env: Env, title: String) -> u32 {
        // Ambil daftar tugas yang sudah ada dari storage, atau buat baru jika kosong [cite: 360, 361, 362]
        let mut tasks: Vec<Task> = env.storage().instance().get(&TASK_DATA).unwrap_or(vec![&env]);
        
        let new_id = tasks.len() + 1;
        let new_task = Task {
            id: new_id,
            title,
            is_completed: false,
        };
        
        // Simpan kembali ke dalam blockchain storage [cite: 363, 381]
        tasks.push_back(new_task);
        env.storage().instance().set(&TASK_DATA, &tasks);
        
        new_id
    }

    // FUNGSI READ: Mengambil semua daftar tugas [cite: 344, 346]
    pub fn get_tasks(env: Env) -> Vec<Task> {
        env.storage().instance().get(&TASK_DATA).unwrap_or(vec![&env])
    }

    // FUNGSI DELETE: Menghapus tugas berdasarkan ID [cite: 350, 352]
    pub fn delete_task(env: Env, id: u32) {
        let tasks: Vec<Task> = env.storage().instance().get(&TASK_DATA).unwrap_or(vec![&env]);
        let mut new_tasks = vec![&env];

        // Filter tugas yang ID-nya tidak sama dengan yang ingin dihapus
        for task in tasks.iter() {
            if task.id != id {
                new_tasks.push_back(task);
            }
        }

        // Update storage dengan daftar tugas yang baru [cite: 381]
        env.storage().instance().set(&TASK_DATA, &new_tasks);
    }
}