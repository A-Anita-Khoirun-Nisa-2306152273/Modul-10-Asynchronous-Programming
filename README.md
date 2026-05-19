# Review Modul 10 - Asynchronous Programming
## Tutorial 1: Timer
### Eksperimen 1.2: Understanding How it Works
#### Hasil Eksekusi Terminal
```text
hey hey
howdy!
done!

```
![With Drop](image.png)

![With Drop](image1.png)

#### Analisis dan Penjelasan
Mengapa urutan keluaran yang muncul di terminal menjadi `hey hey` -> `howdy!` -> `done!`? Berikut adalah analisis fungsional mengenai alur eksekusi asinkronus tersebut:

1. **Sifat Non-blocking dari `spawner.spawn**`:
Ketika fungsi `spawner.spawn(...)` dipanggil, ia tidak langsung mengeksekusi blok kode asinkronus (`async block`) yang berada di dalamnya saat itu juga. Fungsi `spawn` ini hanya bertugas untuk membungkus kode tersebut menjadi sebuah *task* baru dan memasukkannya ke dalam antrean task (*task queue*) milik `Executor`. Setelah itu, alur program langsung melanjutkan eksekusi ke baris berikutnya tanpa menunggu (*non-blocking*). Oleh karena itu, perintah cetak teks baru (`hey hey`) yang diletakkan tepat di bawah `spawner.spawn` dieksekusi terlebih dahulu.
2. **Peran Alur Kendali `Executor**`:
Teks `howdy!` baru dicetak setelah alur kendali fungsi utama (`main`) mencapai baris `executor.run()`. Di sinilah `Executor` mulai aktif mengambil *task-task* yang berada di dalam antrean untuk dijalankan pada urutan pertama. Itulah alasan mengapa `howdy!` muncul setelah `hey hey`.
3. **Penundaan Asinkronus dengan `TimerFuture**`:
Di dalam blok asinkronus tersebut, terdapat pemanggilan `TimerFuture::new(Duration::from_secs(2)).await;`. Ketika *future* ini dipanggil menggunakan kata kunci `.await`, *task* tersebut akan ditangguhkan (*yield*) secara asinkronus untuk memberikan kesempatan bagi CPU mengeksekusi operasi lain selagi menunggu waktu 2 detik selesai. Setelah waktu tunggu habis, mekanisme *waker* akan dipicu untuk memberi tahu `Executor` bahwa *task* tersebut siap dilanjutkan. `Executor` kemudian mengeksekusi sisa baris kode di dalam blok, sehingga teks `done!` tercetak paling terakhir.

### Eksperimen 1.3: Multiple Spawn and Removing Drop

#### Hasil Eksekusi Terminal (Ketika `drop(spawner);` Dihapus)
Program akan mencetak teks dari *multiple spawn* yang dijalankan, namun setelah semua *task* selesai diproses, **program tidak kunjung selesai (menggantung/hang)** dan terminal tidak kembali ke baris perintah baru. Kita harus menghentikannya secara manual menggunakan `Ctrl + C`.

#### Analisis dan Penjelasan

1. **Mengapa Program Menggantung saat `drop(spawner)` Dihapus?**
   Mekanisme `Executor` yang kita bangun menggunakan pola antrean *task* yang mengandalkan saluran komunikasi (*channel*). Fungsi `executor.run()` menggunakan perulangan (`while let Ok(task) = ready_queue.recv()`) untuk terus-menerus mendengarkan dan mengambil *task* baru yang masuk ke dalam antrean.
   
   Saluran (*channel*) ini hanya akan mendeteksi bahwa data telah habis dan menutup koneksinya (`recv()` mengembalikan `Err`) jika **semua instansi `Spawner` (sender) yang memegang *channel* tersebut telah dihancurkan atau di-`drop`**. Jika kita menghapus baris `drop(spawner);` di fungsi `main`, objek `spawner` asli akan tetap hidup di memori. Akibatnya, `Executor` akan terus terjebak menunggu kemungkinan adanya *task* baru yang akan dikirim oleh `spawner`, sehingga perulangan tidak pernah berhenti dan program menggantung.

2. **Mengapa Menyertakan `drop(spawner)` Membuat Program Selesai?**
   Dengan memanggil `drop(spawner);` tepat setelah kita selesai memasukkan (*spawning*) semua *task*, kita secara sadar memberi tahu program bahwa tidak ada lagi *task* baru yang akan dimasukkan ke dalam antrean dari `spawner` utama tersebut. 
   
   Ketika semua *task* yang sudah terlanjur mengantre selesai dieksekusi oleh `Executor` dan instansi `spawner` sudah di-`drop`, saluran komunikasi (*channel*) otomatis akan menutup. Begitu *channel* tutup, fungsi `ready_queue.recv()` akan mengembalikan *error* yang menandakan antrean kosong permanen, sehingga perulangan `while let` pada `Executor` dapat berhenti dengan bersih dan program selesai berjalan.

![With Drop](image2.png)
