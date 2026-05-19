# Review Modul 10 - Asynchronous Programming
## Tutorial 1: Timer
#### Hasil Eksekusi Terminal
```text
hey hey
howdy!
done!

```
![alt text](image.png)

![alt text](image1.png)

#### Analisis dan Penjelasan
Mengapa urutan keluaran yang muncul di terminal menjadi `hey hey` -> `howdy!` -> `done!`? Berikut adalah analisis fungsional mengenai alur eksekusi asinkronus tersebut:

1. **Sifat Non-blocking dari `spawner.spawn**`:
Ketika fungsi `spawner.spawn(...)` dipanggil, ia tidak langsung mengeksekusi blok kode asinkronus (`async block`) yang berada di dalamnya saat itu juga. Fungsi `spawn` ini hanya bertugas untuk membungkus kode tersebut menjadi sebuah *task* baru dan memasukkannya ke dalam antrean task (*task queue*) milik `Executor`. Setelah itu, alur program langsung melanjutkan eksekusi ke baris berikutnya tanpa menunggu (*non-blocking*). Oleh karena itu, perintah cetak teks baru (`hey hey`) yang diletakkan tepat di bawah `spawner.spawn` dieksekusi terlebih dahulu.
2. **Peran Alur Kendali `Executor**`:
Teks `howdy!` baru dicetak setelah alur kendali fungsi utama (`main`) mencapai baris `executor.run()`. Di sinilah `Executor` mulai aktif mengambil *task-task* yang berada di dalam antrean untuk dijalankan pada urutan pertama. Itulah alasan mengapa `howdy!` muncul setelah `hey hey`.
3. **Penundaan Asinkronus dengan `TimerFuture**`:
Di dalam blok asinkronus tersebut, terdapat pemanggilan `TimerFuture::new(Duration::from_secs(2)).await;`. Ketika *future* ini dipanggil menggunakan kata kunci `.await`, *task* tersebut akan ditangguhkan (*yield*) secara asinkronus untuk memberikan kesempatan bagi CPU mengeksekusi operasi lain selagi menunggu waktu 2 detik selesai. Setelah waktu tunggu habis, mekanisme *waker* akan dipicu untuk memberi tahu `Executor` bahwa *task* tersebut siap dilanjutkan. `Executor` kemudian mengeksekusi sisa baris kode di dalam blok, sehingga teks `done!` tercetak paling terakhir.
