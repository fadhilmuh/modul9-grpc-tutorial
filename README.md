# Reflection

1. What are the key differences between unary, server streaming, and bi-directional streaming RPC (Remote Procedure Call) methods, and in what scenarios would each be most suitable?

    Perbedaan utama antara metode RPC (Remote Procedure Call) unary, server streaming, dan bi-directional streaming, serta situasi terbaik untuk masing-masing adalah:
    - Dalam unary streaming, klien hanya mengirimkan data ke server sekali dalam jumlah kecil.
    - Pada server streaming, server mengirimkan data dalam volume besar secara berkelanjutan kepada klien.
    - Streaming dua arah memungkinkan klien dan server untuk bertukar data secara aktif, seperti pada server streaming.

    Unary streaming cocok untuk autentikasi atau pengiriman data form sederhana. Server streaming berguna untuk mengirimkan data besar seperti update harga saham atau berita. Sedangkan streaming dua arah sangat sesuai untuk interaksi langsung seperti pengeditan kolaboratif yang dimiliki oleh google docs atau canva, permainan online, atau chatbots.


2. What are the potential security considerations involved in implementing a gRPC service in Rust, particularly regarding authentication, authorization, and data encryption?

    Dalam implementasi gRPC dengan Rust, terdapat pertimbangan keamanan yang perlu diperhatikan, terutama dalam hal otentikasi, otorisasi, dan enkripsi data termasuk validasi otentikasi dan otorisasi untuk setiap fragmen data yang dikirim, serta enkripsi setiap segmen data untuk menjaga keamanan.

3. What are the potential challenges or issues that may arise when handling bidirectional streaming in Rust gRPC, especially in scenarios like chat applications?

    Dalam menghadapi streaming dua arah dalam gRPC Rust, seperti pada aplikasi chat, beberapa tantangan yang mungkin timbul meliputi kesinkronan pesan yang buruk, potensi *deadlock*, risiko overflow buffer, manajemen koneksi tidak stabil, atau perlunya perhatian khusus pada concurrency dan keamanan.

4. What are the advantages and disadvantages of using the tokio_stream::wrappers::ReceiverStream for streaming responses in Rust gRPC services?

    Penggunaan tokio_stream::wrappers::ReceiverStream dalam streaming respons dalam layanan gRPC Rust memberikan keleluasaan dengan dukungan untuk berbagai jenis receiver, tetapi juga bisa menambah kompleksitas dan overhead kecil terutama bagi yang belum terbiasa dengan asinkronisitas dan konsep Tokio.

5. In what ways could the Rust gRPC code be structured to facilitate code reuse and modularity, promoting maintainability and extensibility over time?

    Untuk meningkatkan pemeliharaan dan pengembangan kode dalam layanan gRPC Rust, struktur yang tepat sangat penting. Menggunakan protokol .proto dan mengintegrasikannya dengan interface yang memungkinkan implementasi kelas-kelas dalam Rust dapat memudahkan penambahan fitur dan perubahan kode.

6. In the MyPaymentService implementation, what additional steps might be necessary to handle more complex payment processing logic?

    Dalam menghadapi logika pemrosesan pembayaran yang kompleks di MyPaymentService, beberapa langkah tambahan yang mungkin diperlukan termasuk validasi dan penanganan kesalahan yang teliti, integrasi dengan gateway pembayaran eksternal, manajemen status transaksi, penegakan aturan bisnis, dan pengujian yang mendalam.

7. What impact does the adoption of gRPC as a communication protocol have on the overall architecture and design of distributed systems, particularly in terms of interoperability with other technologies and platforms?

    Penggunaan gRPC memungkinkan komunikasi yang lebih efisien antarmodul tanpa perlu memikirkan akses melalui protokol HTTP konvensional, yang memudahkan fungsi pemanggilan dari server dan menyederhanakan konektivitas serta operasi antarberbagai teknologi dan platform.

8. What are the advantages and disadvantages of using HTTP/2, the underlying protocol for gRPC, compared to HTTP/1.1 or HTTP/1.1 with WebSocket for REST APIs?

    HTTP/2 mendukung banyak permintaan dan respons dalam satu koneksi yang lebih efisien untuk data besar, meskipun bisa menimbulkan biaya overhead yang lebih tinggi dibandingkan dengan HTTP/1.1. WebSocket lebih baik untuk komunikasi real-time dibandingkan dengan HTTP/2 yang lebih cocok untuk REST API.

9. How does the request-response model of REST APIs contrast with the bidirectional streaming capabilities of gRPC in terms of real-time communication and responsiveness?

    Model permintaan-respons REST hanya satu arah, sedangkan gRPC mendukung streaming dua arah, memungkinkan komunikasi real-time dan responsif, dengan kemampuan mengirim dan menerima data secara asinkron.

10. What are the implications of the schema-based approach of gRPC, using Protocol Buffers, compared to the more flexible, schema-less nature of JSON in REST API payloads?

    Penggunaan Protocol Buffers dalam gRPC menawarkan manfaat dalam efisiensi, keandalan, dan keamanan dalam pertukaran data antar layanan, meskipun GRPC mungkin lebih kompleks dalam pembelajaran. JSON dalam REST API menawarkan fleksibilitas struktur data yang lebih besar tetapi dengan efisiensi dan konsistensi data yang lebih rendah. Pilihan antara keduanya tergantung pada kebutuhan aplikasi, dengan GRPC lebih cocok untuk aplikasi yang menuntut kecepatan tinggi dan keamanan data. Sedangkan REST API lebih sesuai untuk fleksibilitas struktur data dan kemudahan belajar.