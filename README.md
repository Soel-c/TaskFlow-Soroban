# TaskFlow-Soroban 🚀

A decentralized task management application built on the **Stellar Blockchain** using **Soroban Smart Contracts**.

---

## 📝 Deskripsi Proyek
**TaskFlow-Soroban** adalah platform manajemen tugas (To-Do List) berbasis Web3 yang dirancang untuk memberikan transparansi dan keamanan data penuh kepada pengguna. Berbeda dengan aplikasi manajemen tugas tradisional yang menyimpan data di server terpusat, TaskFlow menyimpan setiap entri tugas langsung ke dalam ledger blockchain Stellar melalui smart contract yang ditulis dalam bahasa pemrograman Rust.

Aplikasi ini merupakan hasil modifikasi dari workshop "Build on Stellar" yang diselenggarakan oleh **Rise In** bekerja sama dengan **Telkom University**.

## 👁️ Visi
Membangun ekosistem produktivitas yang berdaulat, di mana pengguna memiliki kontrol penuh atas data mereka tanpa risiko manipulasi dari pihak ketiga, sambil memanfaatkan efisiensi biaya dan kecepatan transaksi jaringan Stellar.

## ✨ List Fitur Proyek
* **Decentralized Storage**: Menyimpan tugas menggunakan sistem *key-value storage* pada instance smart contract.
* **Task Management (CRUD)**:
    * **Add Task**: Membuat catatan tugas baru dengan ID unik.
    * **View Tasks**: Mengambil daftar tugas secara real-time langsung dari blockchain.
    * **Delete Task**: Menghapus tugas yang sudah tidak relevan dari ledger.
* **Gas-Efficient**: Dioptimalkan untuk berjalan dengan biaya transaksi (gas fee) yang sangat rendah menggunakan token XLM.
* **Secure Execution**: Logika aplikasi dijalankan secara otomatis oleh Soroban tanpa perantara manusia.

## 🛠️ Informasi Teknis & Deployment
* **Smart Contract ID**: `CANLIO5VXZM4VO3KUP3Q3GPJXKQ5KQBGKCN5X6PL6TVZCPGRB4EOVYHB`
* **Network**: Stellar Testnet
* **Language**: Rust (Smart Contract) & Soroban SDK
* **Tools**: Stellar CLI, Freighter Wallet, Soroban Studio

## 🚀 Cara Menjalankan Secara Lokal

1. **Clone Repository**
   ```bash
   git clone [https://github.com/Soelc/TaskFlow-Soroban.git](https://github.com/Soel-c/TaskFlow-Soroban.git)
   cd TaskFlow-Soroban
