# Rey

Rey, oy sayim islemlerinin merkeziyetsiz sekilde, hizlica ve seffafca sayilabilmesi icin yayinlanmis bir projedir.

### En Hizli Web Server

https://web-frameworks-benchmark.netlify.app/result

- https://actix.rs/
- salvo

### En dusuk memory kullanimi

https://pkolaczk.github.io/memory-consumption-of-async/

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

<s>Rust-salvo dokumantasyonu iyi oldugu icin secildi, ama `actix` de bir alternatif hala.</s>
https://github.com/programatik29/rust-web-benchmarks/blob/master/result/hello-world.md
Buraya gore actix-web daha hizli. On edenle actix-web uzerinden devam ediyorum.

<s>Dragonfy db olarak secildi, cunku rust-salvo ile kullanilabilir ve hizli.</s>
Database kullanmamaya karar verdim, bunun butun verinin dogrudan dosyaya yazilmasini daha mantikli buluyorum.

Database kullanmayip 
Farkettigim ilk bulgu, nginx sunucusunun, compile edilmis rust suncuusundan cok daha yavas oldugu. Bu durumda sistemi dogrudan onune bir proxy koymadan sunmak daha mantikli.

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

Bu durumda ortalama 5.5 MB/saniye, 5.5*8 = 44 Mbps upload hizi gerekiyor. Bu benim su an aldigim internet hizinin sinirlari icerisinde(50mbit/s). Bunu da sart olmasa da test etmenin bir yolunu bulmak gerekiyor.



Saniyede 1GBPS up/down internet baglantisi hizi varsayalim.

## Mimari

Su an icin en buyuk problem bu kadar verinin guvenli bir sekilde diske yazilip nasil indexleneceginden geciyor. Bu problemi 2 ayri adimda cozmek mumkun gibi gorunuyor.

1 - Verilerin diske aninda yazilmasi: Gonderilen veriler, pusula fotografi olsun, pusulanin okunmasi verisi olsun, sadece 2 ayri sekilde indexlenmesi gerekiyor.

- Sayan Kullanciya gore: Oy sayan kullanicinin(kimligi anonim olsa bile), kimlerin hangi sandiklari saydigini bilmek onemli. Buna gore kisilere puan verilebilir ve sayimlarina daha cok guvenilebilir.
- Sandik numarasina gore. Oncelikle sandik fotograflarinin hepsi "unassigned" isminde bir tagin altinda olacak, ardindan belirli sayida insan ayni veriyi girince, kendi sandik no'sundaki taga tasinacak.

### Veri Depolama

Yukarida anlatilan mimaride veri depolamak icin [sled](https://docs.rs/sled/latest/sled/index.html#) kullanilacak.

### Tag Sistemi

Kullanilacak ana data yapisi:

`let mut map: HashMap<Vec<String>, String> = HashMap::new();`

Seklinde olacaktir. Burda key olarak kullandigimiz Vec<String> tipi, bir tag listesini temsil ediyor. 
Ornegin sayana  gore tag listesi:

```
["sayim","kullanici123123", "u452-dsfsdf-234234-234"]  => Bu sayim islemi yapan kullanici123123 kullanicisinin, u452-dsfsdf-234234-234 sandik numarali islemi yaptigini gosteriyor.
```
```
["sandik","123123","u452-dsfsdf-234234-234"] => Bu sandiklar arasinda 123123 numarali sandiga ait olan "u452-dsfsdf-234234-234" elemanina denk geliyor. Yani iki ayri sekilde sorgulama yapildiginda da ayni elemana ulasilabiliyor.
```
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
