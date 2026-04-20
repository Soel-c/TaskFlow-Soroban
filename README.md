# TaskFlow-Soroban 🚀

A decentralized task management application built on the **Stellar Blockchain** using **Soroban Smart Contracts**.

---

## 📝 Deskripsi Proyek
**TaskFlow-Soroban** adalah platform manajemen tugas (To-Do List) berbasis Web3 yang dirancang untuk memberikan transparansi dan keamanan data penuh kepada pengguna. [cite_start]Berbeda dengan aplikasi manajemen tugas tradisional yang menyimpan data di server terpusat, TaskFlow menyimpan setiap entri tugas langsung ke dalam ledger blockchain Stellar melalui smart contract yang ditulis dalam bahasa pemrograman Rust[cite: 6, 218].

[cite_start]Aplikasi ini merupakan hasil modifikasi dari workshop "Build on Stellar" yang diselenggarakan oleh **Rise In** bekerja sama dengan **Telkom University**[cite: 1, 3, 7].

## 👁️ Visi
[cite_start]Membangun ekosistem produktivitas yang berdaulat, di mana pengguna memiliki kontrol penuh atas data mereka tanpa risiko manipulasi dari pihak ketiga, sambil memanfaatkan efisiensi biaya dan kecepatan transaksi jaringan Stellar[cite: 192, 193].

## ✨ List Fitur Proyek
* [cite_start]**Decentralized Storage**: Menyimpan tugas menggunakan sistem *key-value storage* pada instance smart contract[cite: 359, 360].
* **Task Management (CRUD)**:
    * [cite_start]**Add Task**: Membuat catatan tugas baru dengan ID unik[cite: 349].
    * [cite_start]**View Tasks**: Mengambil daftar tugas secara real-time langsung dari blockchain[cite: 346].
    * [cite_start]**Delete Task**: Menghapus tugas yang sudah tidak relevan dari ledger[cite: 352].
* [cite_start]**Gas-Efficient**: Dioptimalkan untuk berjalan dengan biaya transaksi (gas fee) yang sangat rendah menggunakan token XLM[cite: 224].
* [cite_start]**Secure Execution**: Logika aplikasi dijalankan secara otomatis oleh Soroban tanpa perantara manusia[cite: 181, 182].

## 🛠️ Informasi Teknis & Deployment
* **Smart Contract ID**: `[MASUKKAN_ID_CONTRACT_ANDA_DI_SINI]`
* [cite_start]**Network**: Stellar Testnet [cite: 385]
* [cite_start]**Language**: Rust (Smart Contract) & Soroban SDK [cite: 218, 312]
* [cite_start]**Tools**: Stellar CLI, Freighter Wallet, Soroban Studio [cite: 232, 399, 426]

## 🚀 Cara Menjalankan Secara Lokal

1. **Clone Repository**
   ```bash
   git clone [https://github.com/username/TaskFlow-Soroban.git](https://github.com/username/TaskFlow-Soroban.git)
   cd TaskFlow-Soroban
