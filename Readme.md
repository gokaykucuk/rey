# Rey

Rey, oy sayim islemlerinin merkeziyetsiz sekilde, hizlica ve seffafca sayilabilmesi icin yayinlanmis bir projedir.

# En Hizli Web Server

https://web-frameworks-benchmark.netlify.app/result

- https://actix.rs/
- salvo

# Mimari

Rey server 3 ana bilesenden olusur,

1 - Islak imzali belge toplama modulu.
2 - Islak imzali belge dijitallestirme modulu.
3 - Sonuclari goruntuleme modulu.


```
cargo run --release
```

# Arastirma

Rust frameworkleri genellikle en hizliliari ve C++ a gore daha anlamasi kolay oldugu icin rust dilini tercih ettim.

Rust-salvo dokumantasyonu iyi oldugu icin secildi, ama `actix` de bir alternatif hala.

Dragonfy db olarak secildi, cunku rust-salvo ile kullanilabilir ve hizli.

Farkettigim ilk bulgu, nginx sunucusunun, compile edilmis rust-salvo suncuusundan cok daha yavas oldugu. Bu durumda sistemi dogrudan onune bir proxy koymadan sunmak daha mantikli.

Docker icerisinden calisirsa yaklasik 4-5 kat daha yavas calisiyor sistem, docker icindeki network ayarlarini degistirerek bu durumu duzeltmek gerekiyor.

## Hesaplar

Butun sistemin kendi bilgisayarim olan Macbook Pro 12 CPU ve 32GB RAM ile calismasi buradaki asil hedef. Bunun icin asagidaki hesaplamalari yaptim.

Turkiye gozonune alindiginda yaklasik 50 milyon secmen olduguna ve 200.000 civari sandik oldugunu kabul ediyorum.

Her sandikta ortalama 1000 oy kullanildigini varsayiyorum. Toplam sacma olabilir ama bu onemli degil.

Secim ve sayim surecinin yaklasik 6 saat surecegini, bu 6 saat boyunca performans dususu yasanmamasi gerektigini varsayiyorum.

Her sandiktan cekilen bir sandik fotografi olmasi gerekiyor. Dagitik bir sistem oldugu icin, her sandiktan birden cok fotograf geldigini varsayalim.

O zaman 200.000 sandik icin 2 milyon fotograf(sandik basina 10 resim, trollemeler vs dahil) toplanmasi gerekiyor. Bu fotograflarin ortalama 1MB oldugunu varsayalim. Bu durumda 2TB fotograf toplanmasi gerekiyor. 2 TB depolama alani benim bilgisayarimin kapasitesini asiyor. Burada harici bir disk kullanmayi dusunuyorum depolama olarak. Biraz performans kaybi yasarim ama, sanmiyorum sorun olacagini. Yine de dusuk oncelikli bir hedef olarak, 500-600 GB civarinda depolanmasini saglayacak bir cozumun uzerinde calislilabilir.

Yaklasik 2TB, hatta fotograflar 1MB den buyuk olabilecegi icin belki daha fazla trafigin kabul edilebilmesi gerekiyor.

2 TB/6 saat = 5.5 MB/saniye

Bu durumda ortalama 5.5 MB/saniye upload hizi gerekiyor. 5.5*8 = 44 Mbps upload hizi gerekiyor. Bu benim su an aldigim internet hizinin sinirlari icerisinde(50mbit/s). Bunu da sart olmasa da test etmenin bir yolunu bulmak gerekiyor.



Saniyede 1GBPS up/down internet baglantisi hizi varsayalim suncu

# Test
## Autotest
```
cargo test
```
## Benchmark
```
For benchmarking the project uses "drill". You can install it with:
```
cargo install drill
```
Then run the command below to start the benchmark:
```
drill --benchmark benchmark.yml
```
