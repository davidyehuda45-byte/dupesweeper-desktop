<div align="center">

# DupeSweeper — डुप्लिकेट फ़ाइल खोजक और सिस्टम क्लीनर
### संस्करण 11.0.0 — एक्सपोर्ट, इतिहास और अनडू, ऑटो-स्कैन, डिस्क विश्लेषक

**तेज़, हल्का और सुरक्षित मल्टी-प्लेटफ़ॉर्म डुप्लिकेट फ़ाइल फाइंडर एवं सिस्टम जंक क्लीनर**

[![Bahasa Indonesia](https://img.shields.io/badge/Language-Bahasa%20Indonesia-lightgrey?style=flat-square)](../../README.md)
[![English](https://img.shields.io/badge/Language-English-lightgrey?style=flat-square)](README.en.md)
[![简体中文](https://img.shields.io/badge/Language-简体中文-lightgrey?style=flat-square)](README.zh.md)
[![हिन्दी](https://img.shields.io/badge/Language-हिन्दी-blue?style=flat-square)](README.hi.md)

<p align="center">
  <b>भाषा चुनें / Select Language:</b><br>
  <a href="../../README.md"><b>Bahasa Indonesia</b></a> &middot;
  <a href="README.en.md"><b>English</b></a> &middot;
  <a href="README.zh.md"><b>简体中文</b></a> &middot;
  <a href="README.hi.md"><b>हिन्दी</b></a>
</p>

</div>

---

**DupeSweeper** एक आधुनिक, तीव्र और 100% ऑफ़लाइन डेस्कटॉप एप्लिकेशन है जो **Windows, Linux और macOS** का समर्थन करता है। यह उच्च-गति डुप्लिकेट फ़ाइल खोज और सिस्टम जंक क्लीनअप को एक ही सुव्यवस्थित एप्लिकेशन में जोड़ता है:

1. **Duplicate Finder मोड** — **BLAKE3** क्रिप्टोग्राफ़िक कंटेंट हैशिंग का उपयोग करके वास्तविक फ़ाइल सामग्री के आधार पर डुप्लिकेट फ़ाइलों की पहचान और सफ़ाई करता है।
2. **General Cleanup मोड** — सिस्टम जंक और सामान्य एप्लिकेशन कैश (अस्थायी फ़ाइलें, ब्राउज़र कैश, थंबनेल डेटाबेस, पुराने लॉग, Downloads में पुराने इंस्टॉलर, तथा डेवलपर/कंज़्यूमर टूल्स कैश) को स्कैन और साफ़ करता है।
3. **Disk Analyzer मोड** — किसी फ़ोल्डर की सामग्री को आकार के अनुसार क्रमबद्ध कर, ड्रिल-डाउन नेविगेशन के साथ ब्राउज़ करता है, ताकि पता चल सके कि डिस्क स्थान कहाँ उपयोग हो रहा है।

यह एप्लिकेशन **5.5 MB से कम आकार की एकल स्टैंडअलोन बाइनरी** में संकलित होता है, जिसमें कोई अतिरिक्त रनटाइम निर्भरता नहीं है, जटिल इंस्टॉलेशन नहीं है, कोई विज्ञापन नहीं है, और **क्लाउड पर कोई डेटा नहीं भेजा जाता**।

---

## सीधे डाउनलोड करें और चलाएँ (बिना इंस्टॉलेशन, बिना Git Clone)

DupeSweeper एक पोर्टेबल स्टैंडअलोन डेस्कटॉप एप्लिकेशन के रूप में वितरित किया जाता है। आपको इस रिपॉजिटरी को क्लोन करने या Rust कंपाइलर इंस्टॉल करने की आवश्यकता **नहीं** है। बस अपने प्लेटफ़ॉर्म के लिए बाइनरी डाउनलोड करें और चलाएँ।

| ऑपरेटिंग सिस्टम | एप्लिकेशन फ़ाइल | आकार | चलाने का तरीका |
|---|---|:---:|---|
| **Windows (इंस्टॉलर)** | [DupeSweeper-Setup.exe](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases/download/v6.0.0/DupeSweeper-Setup.exe) | ~5.0 MB | अनुशंसित। डाउनलोड कर इंस्टॉल करें। Desktop व Start Menu पर स्वतः "DupeSweeper" शॉर्टकट बनाता है। |
| **Windows (पोर्टेबल)** | [DupeSweeper.exe](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases/download/v6.0.0/DupeSweeper.exe) | ~5.0 MB | बिना इंस्टॉलेशन के स्टैंडअलोन एक्ज़िक्यूटेबल। डाउनलोड कर सीधे चलाएँ। |
| **Linux (x64)** | [dupesweeper-linux-x86_64](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases/download/v6.0.0/dupesweeper-linux-x86_64) | ~11.3 MB | `chmod +x dupesweeper-linux-x86_64` फिर `./dupesweeper-linux-x86_64` चलाएँ |
| **macOS (Universal)** | [dupesweeper-macos-universal](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases/download/v6.0.0/dupesweeper-macos-universal) | ~4.5 MB | `chmod +x dupesweeper-macos-universal` फिर `./dupesweeper-macos-universal` चलाएँ |

> संपूर्ण रिलीज़ संग्रह, संस्करण इतिहास और चेंजलॉग [GitHub Releases](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases) पर उपलब्ध हैं।

---

### Windows SmartScreen एवं एंटीवायरस सुरक्षा संबंधी जानकारी

पहली बार `DupeSweeper-Setup.exe` या `DupeSweeper.exe` डाउनलोड कर चलाने पर, Windows Defender / SmartScreen निम्न चेतावनी दिखा सकता है:

> "Windows protected your PC"
> Microsoft Defender SmartScreen prevented an unrecognized app from starting.

**यह चेतावनी क्यों दिखाई देती है:**
1. **मुफ़्त ओपन-सोर्स सॉफ़्टवेयर।** DupeSweeper 100% मुफ़्त और ओपन-सोर्स है। SmartScreen उन `.exe` फ़ाइलों को चिह्नित करता है जिनके पास भुगतान किया हुआ EV Code Signing Certificate (लगभग $300–500 प्रति वर्ष) नहीं होता।
2. **नई रिलीज़ की प्रतिष्ठा अभी स्थापित नहीं हुई।** SmartScreen डाउनलोड की संचयी प्रतिष्ठा प्रणाली पर निर्भर करता है। पर्याप्त उपयोगकर्ताओं द्वारा डाउनलोड होने तक हर नए संस्करण को "अपरिचित" माना जाता है।

**इसे चलाने का तरीका (केवल पहली बार):**
1. नीली SmartScreen विंडो में "More info" पर क्लिक करें।
2. दिखाई देने वाले "Run anyway" बटन पर क्लिक करें।
3. DupeSweeper तुरंत खुल जाएगा।

> **सुरक्षा एवं गोपनीयता की गारंटी:**
> - **100% ऑफ़लाइन, ज़ीरो टेलीमेट्री।** एप्लिकेशन इंटरनेट से कभी कनेक्ट नहीं होता, कोई एनालिटिक्स नहीं भेजता, और उपयोगकर्ताओं को ट्रैक नहीं करता।
> - **एडमिनिस्ट्रेटर अधिकारों की आवश्यकता नहीं।** मानक उपयोगकर्ता अनुमति (`asInvoker`) पर चलता है और कभी भी UAC एलिवेशन नहीं माँगता।
> - **पारदर्शी और ओपन-सोर्स।** संपूर्ण सोर्स कोड इस रिपॉजिटरी में स्वतंत्र रूप से ऑडिट किया जा सकता है। सभी प्लेटफ़ॉर्म की बाइनरी GitHub Actions CI द्वारा स्वचालित और reproducible रूप से बनाई जाती हैं।

---

## v11.0.0 में नया क्या है

1. **रिपोर्ट एक्सपोर्ट (CSV/JSON)** — डुप्लिकेट स्कैन परिणाम या क्लीनअप विश्लेषण को डिलीट करने से पहले ऑडिट अथवा बैकअप हेतु CSV या JSON फ़ाइल में एक्सपोर्ट करें।
2. **इतिहास एवं अनडू (History & Undo)** — हर क्लीनअप सत्र स्थानीय रूप से दर्ज होता है। Recycle Bin में भेजी गई फ़ाइलों को DupeSweeper सीधे एप्लिकेशन के भीतर से उनके मूल स्थान पर वापस (undo) ला सकता है।
3. **शेड्यूल्ड ऑटो-स्कैन** — एप्लिकेशन खुला रहने के दौरान, एक निर्धारित अंतराल (30 मिनट से 24 घंटे तक) पर अंतिम उपयोग किए गए फ़ोल्डरों को स्वतः पुनः स्कैन करता है।
4. **डिस्क विश्लेषक (Disk Analyzer)** — एक नया टैब जो किसी डायरेक्टरी के भीतर सबसे बड़े फ़ोल्डर व फ़ाइलों को ब्रेडक्रंब-आधारित ड्रिल-डाउन नेविगेशन के साथ दिखाता है।

---

## क्रॉस-प्लेटफ़ॉर्म समर्थन

### प्लेटफ़ॉर्म एब्स्ट्रैक्शन लेयर (`src/platform/`)
- **एकल सोर्स कोडबेस।** मुख्य डिडुप्लिकेशन इंजन और immediate-mode GUI पूर्णतः पोर्टेबल हैं। ऑपरेटिंग सिस्टम के अंतर `src/platform/` में स्पष्ट रूप से पृथक किए गए हैं:
  - `src/platform/windows.rs` — Win32 Shell32 Recycle Bin क्वेरी व खाली करना, `%TEMP%` / `%LOCALAPPDATA%` पाथ रिज़ॉल्वर, Windows Explorer एकीकरण।
  - `src/platform/linux.rs` — Freedesktop.org Trash Specification (`~/.local/share/Trash`), मानक `/tmp`, `/var/tmp`, `~/.cache` रिज़ॉल्वर, तथा `xdg-open`।
  - `src/platform/macos.rs` — Finder Trash (`~/.Trash`), `~/Library/Caches`, `~/Library/Application Support`, QuickLook थंबनेल कैश, तथा `open -R`।
- **`dirs` crate एकीकरण।** हार्डकोडेड Windows environment variables के स्थान पर मानक क्रॉस-प्लेटफ़ॉर्म पाथ रिज़ॉल्वर (`dirs::cache_dir()`, `dirs::config_dir()`, `dirs::download_dir()`, `dirs::home_dir()`) का उपयोग।

### मल्टी-प्लेटफ़ॉर्म क्लीनअप पाथ मैट्रिक्स

| श्रेणी | Windows | Linux | macOS |
|---|---|---|---|
| अस्थायी फ़ाइलें | `%TEMP%`, `C:\Windows\Temp` | `/tmp`, `/var/tmp` | `/private/var/folders`, `/tmp`, `~/Library/Caches` |
| Chrome कैश | `%LOCALAPPDATA%\Google\Chrome\User Data\Default\Cache` | `~/.cache/google-chrome/Default/Cache` | `~/Library/Caches/Google/Chrome/Default/Cache` |
| Firefox कैश | `%APPDATA%\Mozilla\Firefox\Profiles\*\cache2` | `~/.cache/mozilla/firefox/*/cache2` | `~/Library/Caches/Firefox/Profiles/*/cache2` |
| थंबनेल कैश | `thumbcache_*.db` (Explorer) | `~/.cache/thumbnails` | `~/Library/Caches/com.apple.QuickLook.thumbnailcache` |
| Recycle Bin / Trash | Windows Recycle Bin (Win32 Shell32) | `~/.local/share/Trash` (Freedesktop) | `~/.Trash` (Finder) |
| Downloads में पुराने इंस्टॉलर | `*.exe`, `*.msi` | `*.deb`, `*.rpm`, `*.appimage`, `*.tar.gz` | `*.dmg`, `*.pkg` |
| npm कैश | `%APPDATA%\npm-cache` | `~/.npm` | `~/.npm` |
| pip कैश | `%LOCALAPPDATA%\pip\Cache` | `~/.cache/pip` | `~/Library/Caches/pip` |
| VS Code कैश | `%APPDATA%\Code\Cache` | `~/.config/Code/Cache` | `~/Library/Application Support/Code/Cache` |
| Cargo registry कैश | `~/.cargo/registry/cache` | `~/.cargo/registry/cache` | `~/.cargo/registry/cache` |
| सामान्य ऐप्स | Discord, Spotify | Discord, Spotify | Discord, Spotify |

### स्वचालित मल्टी-प्लेटफ़ॉर्म CI/CD
- `.github/workflows/ci.yml` में एक GitHub Actions वर्कफ़्लो कॉन्फ़िगरेशन उपलब्ध है।
- हर push/pull request पर `windows-latest`, `ubuntu-latest`, तथा `macos-latest` पर स्वतः `cargo test` एवं `cargo build --release` चलाया जाता है।

---

## General Cleanup मोड की विशेषताएँ

शीर्ष नेविगेशन टैब से सीधे चुना जाने वाला एक अलग मोड: "Duplicate File खोजें" बनाम "System Junk साफ़ करें"।

### साफ़ की जाने वाली जंक फ़ाइल श्रेणियाँ

1. **अस्थायी फ़ाइलें** — इंस्टॉलर या क्रैश हुए एप्लिकेशन की बची हुई अस्थायी फ़ाइलें (`*.tmp`, `*.temp`)। स्थिति: सुरक्षित, डिफ़ॉल्ट रूप से चयनित।
2. **ब्राउज़र कैश (Chrome, Edge, Firefox)** — केवल शुद्ध कैश डेटा (`Cache`, `Code Cache`, `GPUCache`, `CacheStorage`)। सुरक्षा सिद्धांत: कुकीज़, सेव्ड लॉगिन, ब्राउज़िंग हिस्ट्री, या बुकमार्क को कभी नहीं छूता। स्थिति: सुरक्षित, डिफ़ॉल्ट रूप से चयनित।
3. **पुराने लॉग व क्रैश डंप (30 दिन से अधिक)** — एरर लॉग (`*.log`), क्रैश डंप (`*.dmp`), और पुरानी सिस्टम एरर रिपोर्ट। स्थिति: सुरक्षित, डिफ़ॉल्ट रूप से चयनित।
4. **थंबनेल कैश** — फ़ाइल मैनेजर की थंबनेल प्रीव्यू कैश डेटाबेस, आवश्यकता पड़ने पर सिस्टम द्वारा स्वतः पुनः जनरेट होती है। स्थिति: सुरक्षित, डिफ़ॉल्ट रूप से चयनित।
5. **Recycle Bin / Trash की सामग्री** — सभी ड्राइव्स की कुल क्षमता पढ़ता है और पुष्टि के साथ खाली करने का विकल्प देता है। स्थिति: सुरक्षित, डिफ़ॉल्ट रूप से चयनित।
6. **Downloads में पुराने इंस्टॉलर (60 दिन से अधिक)** — 60 दिन से अधिक पुरानी इंस्टॉलर फ़ाइलें (`*.exe`, `*.msi`, `*.deb`, `*.rpm`, `*.AppImage`, `*.dmg`, `*.pkg`)। अभी भी आवश्यक फ़ाइलों को गलती से हटने से बचाने हेतु डिफ़ॉल्ट रूप से अचयनित। स्थिति: मैन्युअल समीक्षा आवश्यक।
7. **Developer Tools कैश (npm, pip, VS Code, Cargo)** — ग्लोबल पैकेज मैनेजर कैश, आवश्यकता पड़ने पर टूल्स द्वारा स्वतः पुनः डाउनलोड होता है। स्थिति: सुरक्षित, डिफ़ॉल्ट रूप से चयनित।
8. **सामान्य एप्लिकेशन कैश (Discord, Spotify)** — Discord व Spotify का स्थानीय स्टोरेज कैश, अगली बार उपयोग करने पर स्वतः रीफ्रेश होता है। स्थिति: सुरक्षित, डिफ़ॉल्ट रूप से चयनित।

### General Cleanup सुरक्षा विशेषताएँ
- **ग्रेसफुल एरर हैंडलिंग** — किसी अन्य एप्लिकेशन द्वारा लॉक की गई या उपयोग में ली गई फ़ाइलें बिना क्रैश हुए स्वतः स्किप हो जाती हैं।
- **ड्राई-रन साइज़ कैलकुलेशन** — क्लीनअप बटन दबाने से पहले मुक्त होने वाली अनुमानित क्षमता की गणना।
- **डिटेल ड्रॉअर** — प्रत्येक श्रेणी कार्ड को विस्तृत कर उसके अंदर विशिष्ट फ़ाइलों की सूची देखी जा सकती है।
- **ऑडिट लॉगिंग** — सभी क्लीनअप गतिविधियाँ एक ऑडिट लॉग फ़ाइल (`logs/dupesweeper_audit_*.txt`) में दर्ज होती हैं।

---

## Template Fingerprint डिटेक्शन (Duplicate मोड)

DupeSweeper एक बुद्धिमान प्रणाली से लैस है जो कई प्रोजेक्ट्स में समान रहने वाली फ्रेमवर्क स्टार्टर टेम्पलेट फ़ाइलों (Laravel, Next.js, Vite React, Vite Vue) को पहचानती है।

### श्रेणी C — Documentation Template (सुरक्षित रूप से हटाई जा सकती हैं)
- स्टार्टर किट की डिफ़ॉल्ट डॉक्यूमेंटेशन/मेटाडेटा फ़ाइलें (`README.md`, `.gitignore`, `welcome.blade.php`, `vite.svg`, `react.svg` इत्यादि)।
- बैज: "Default Template ([Framework])"।
- व्यवहार: auto-suggest क्लीनअप में शामिल की जाती हैं।

### श्रेणी D — Functional Template (Auto-Selection से सुरक्षित)
- फ्रेमवर्क की अपरिवर्तित फ़ंक्शनल/रनटाइम फ़ाइलें (`0001_01_01_000000_create_users_table.php`, `DatabaseSeeder.php`, `routes/web.php`, `src/app/page.tsx`, `vite.config.js` इत्यादि)।
- बैज: "Stock [Framework] (Unmodified)"।
- व्यवहार: सभी सिलेक्शन स्ट्रैटेजी में auto-suggest क्लीनअप से सुरक्षित, डिफ़ॉल्ट रूप से अचयनित।

---

## प्रदर्शन (Performance) विशेषताएँ

- **बैकग्राउंड बल्क डिलीशन (`DeleteWorker`)** — फ़ाइल डिलीशन non-blocking crossbeam channel के माध्यम से एक समर्पित बैकग्राउंड थ्रेड पर चलता है, UI कभी फ़्रीज़ नहीं होता। किसी भी समय सुरक्षित रूप से रोकने हेतु atomic flag आधारित "Cancel" बटन उपलब्ध है।
- **वर्चुअलाइज़्ड / लेज़ी स्क्रॉलिंग (`show_rows`)** — केवल स्क्रीन के व्यूपोर्ट में मौजूद पंक्तियों को रेंडर करता है। 1,00,000+ फ़ाइलों के डेटासेट पर भी स्क्रॉलिंग स्मूथ रहती है।
- **डिफ़ॉल्ट रूप से कोलैप्स्ड** — डुप्लिकेट ग्रुप डिफ़ॉल्ट रूप से कोलैप्स्ड दिखते हैं, साथ में "सभी खोलें" और "सभी बंद करें" नियंत्रण उपलब्ध हैं।
- **एसिंक्रोनस लेज़ी थंबनेल वर्कर** — इमेज थंबनेल डिकोडिंग एक अलग बैकग्राउंड थ्रेड पर होती है, UI थ्रेड पर कोई भार नहीं पड़ता।

---

## श्रेणी वर्गीकरण मैट्रिक्स

| श्रेणी | नाम | उदाहरण | स्कैन व्यवहार | सिलेक्शन व्यवहार | UI बैज |
|---|---|---|---|---|---|
| A | Excluded Directories | `node_modules`, `vendor`, `target`, `.git`, `.next`, `DerivedData`, `.cache` | पूरी तरह स्किप | लागू नहीं | लागू नहीं |
| B | Protected Sensitive Files | `.env`, `*.key`, `*.pem`, `secrets.json`, SSH keys | फिर भी स्कैन व डिटेक्ट होती हैं | डिफ़ॉल्ट अचयनित, auto-select से सुरक्षित | Sensitive |
| C | Documentation Template | `README.md`, `.gitignore`, `welcome.blade.php`, `react.svg` | स्कैन व BLAKE3 से मैच | auto-suggested क्लीनअप हेतु | Default Template ([Framework]) |
| D | Functional Template | Laravel migrations, `DatabaseSeeder.php`, `page.tsx`, `vite.config.js` | स्कैन व BLAKE3 से मैच | डिफ़ॉल्ट अचयनित, auto-select से सुरक्षित | Stock [Framework] (Unmodified) |
| Clean | System & App Junk | Temp files, browser cache, logs, thumbnail, installers | लोकेशन व पैटर्न स्कैन | Safe = ON, Risky = OFF | Safe / Needs Review |

---

## प्रत्येक प्लेटफ़ॉर्म पर बिल्ड व रन करने का तरीका

### Windows
```powershell
# सोर्स से सीधे चलाएँ:
cargo run --release

# स्टैंडअलोन .exe बनाएँ:
cargo build --release
# बाइनरी target/release/dupesweeper.exe पर तैयार मिलेगी
```

### Linux
```bash
# बेसिक GUI निर्भरताएँ इंस्टॉल करें (Ubuntu/Debian):
sudo apt-get install -y libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libasound2-dev

# सीधे चलाएँ:
cargo run --release

# स्टैंडअलोन बाइनरी बनाएँ:
cargo build --release

# वैकल्पिक: पोर्टेबल .AppImage के रूप में पैकेज करें
cargo install cargo-appimage
cargo appimage
```

### macOS
```bash
# सीधे चलाएँ:
cargo run --release

# स्टैंडअलोन बाइनरी बनाएँ:
cargo build --release

# वैकल्पिक: .app / .dmg बंडल के रूप में पैकेज करें
cargo install cargo-bundle
cargo bundle --release
```

---

## परीक्षण परिणाम (Automated Tests)

टेस्ट सूट में Duplicate Finder, General Cleanup, Background Deletion, Cross-Platform Resolution, तथा v11 की नई सुविधाओं (export, history/undo, auto-scan, disk analyzer) के सभी परिदृश्य शामिल हैं:

1. `test_hasher_partial_and_full` — Partial बनाम Full hash की अखंडता की पुष्टि।
2. `test_end_to_end_scanner_detection` — Multi-folder end-to-end scanner की पुष्टि।
3. `test_selection_strategies` — Keep Oldest, Keep Newest, Keep Shortest Path की पुष्टि।
4. `test_action_quarantine_execution` — Quarantine में स्थानांतरण व ऑडिट लॉग रिकॉर्डिंग।
5. `test_zero_byte_files_filtered_by_default` — 0-byte फ़ाइलों की फ़िल्टरिंग।
6. `test_three_way_duplicates_across_nested_subfolders` — नेस्टेड सबफ़ोल्डर में मल्टी-डुप्लिकेट की पुष्टि।
7. `test_excluded_directories_are_skipped` — `node_modules`, `vendor` जैसी डायरेक्टरी स्वतः स्किप होने की पुष्टि।
8. `test_sensitive_files_default_unchecked` — संवेदनशील फ़ाइलें (`.env`, keys) auto-selection से सुरक्षित रहने की पुष्टि।
9. `test_scan_all_toggle_overrides_exclude` — "Scan all" टॉगल exclude list को बायपास करने की पुष्टि।
10. `test_large_dataset_virtualized_performance` — 50,000 ग्रुप (1,50,000 फ़ाइलें) का सिमुलेशन, flat row mapping 50ms से कम समय में बिना frame drop के प्रोसेस।
11. `test_documentation_template_match_is_auto_selected` — Documentation template (श्रेणी C) डिटेक्ट होकर auto-suggest क्लीनअप में शामिल होने की पुष्टि।
12. `test_functional_template_match_is_protected_from_auto_selection` — Functional template (श्रेणी D) सभी strategies व Select All में सुरक्षित रहने की पुष्टि।
13. `test_end_to_end_template_fingerprint_scanner` — श्रेणी C व D को एक साथ डिटेक्ट करने वाला end-to-end परीक्षण।
14. `test_modified_framework_file_no_longer_matches_template` — डेवलपर द्वारा संशोधित template फ़ाइल अब fingerprint से मैच न होने की पुष्टि।
15. `test_locked_file_skipped_gracefully` — किसी अन्य प्रोसेस द्वारा locked फ़ाइल बिना क्रैश हुए स्किप होकर ऑडिट लॉग में दर्ज होने की पुष्टि।
16. `test_installer_category_default_unchecked` — Downloads installer श्रेणी डिफ़ॉल्ट रूप से अचयनित होने की पुष्टि (`SafetyLevel::NeedsReview`)।
17. `test_safe_categories_default_checked` — सुरक्षित श्रेणियाँ (temp, browser cache, logs, thumbnail, recycle bin, dev tools, apps) डिफ़ॉल्ट रूप से चयनित होने की पुष्टि (`SafetyLevel::Safe`)।
18. `test_size_calculation_matches_actual_deletion` — क्लीनअप-पूर्व अनुमानित आकार व वास्तविक मुक्त हुए स्थान के बिल्कुल समान होने की पुष्टि।
19. `test_recycle_bin_query` — Native API के माध्यम से Recycle Bin / Trash स्थिति क्वेरी की सफलता।
20. `test_delete_worker_background_execution` — UI थ्रेड को ब्लॉक किए बिना बैकग्राउंड थ्रेड पर डिलीशन की पुष्टि।
21. `test_delete_worker_cancellation` — Atomic flag के माध्यम से bulk delete के सुरक्षित रद्दीकरण की पुष्टि।
22. `test_cleanup_executor_cancellation` — General cleanup के सुरक्षित रद्दीकरण की पुष्टि।
23. `test_platform_temp_dirs_valid` — क्रॉस-प्लेटफ़ॉर्म temp directories की उपलब्धता की पुष्टि।
24. `test_platform_downloads_installer_patterns` — Installer pattern की पुष्टि (Windows पर `.exe`/`.msi`, Linux पर `.deb`/`.rpm`/`.appimage`, macOS पर `.dmg`/`.pkg`)।
25. `test_platform_dev_tools_cache_dirs` — Developer tools cache directory resolver (npm, pip, VS Code, Cargo) की पुष्टि।
26. `test_platform_consumer_apps_cache_dirs` — Discord व Spotify cache directory resolver की पुष्टि।
27. `test_platform_recycle_bin_query` — Recycle Bin / Trash के आकार व आइटम count क्वेरी की पुष्टि।
28. `test_linux_module_resolvers_do_not_panic` — Linux path resolvers के बिना panic चलने की पुष्टि।
29. `test_macos_module_resolvers_do_not_panic` — macOS path resolvers के बिना panic चलने की पुष्टि।
30. `test_installer_info_and_shortcut_creation` — Windows install info व शॉर्टकट निर्माण की पुष्टि।
31. `test_export_duplicates_csv_and_json_roundtrip` — Duplicate finder परिणामों को CSV व JSON में एक्सपोर्ट करने की पुष्टि।
32. `test_export_cleanup_csv_and_json_roundtrip` — General cleanup परिणामों को CSV व JSON में एक्सपोर्ट करने की पुष्टि।
33. `test_history_entry_is_restorable_only_for_recycle_bin_with_refs` — History entry की undo-योग्यता लॉजिक की पुष्टि।
34. `test_history_action_kind_labels_are_distinct` — History action type लेबल की पुष्टि।
35. `test_auto_scan_interval_seconds_mapping` — Auto-scan interval conversion की पुष्टि।
36. `test_app_settings_serde_roundtrip_in_memory` — Application settings serialization की पुष्टि।
37. `test_folder_analyzer_computes_child_sizes_and_recurses_into_subfolders` — Folder size analyzer की गणना, सहित subfolder की recursive aggregation, की पुष्टि।

सभी 37 परीक्षण सफलतापूर्वक उत्तीर्ण (100%)।

```bash
cargo test
```
