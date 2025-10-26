# 🎯 Zed Coding Tracker Extension

> Track your coding time in Zed Editor with persistent statistics and automatic session management.

![Version](https://img.shields.io/badge/version-0.2.0-blue)
![Rust](https://img.shields.io/badge/rust-1.70+-orange)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey)

---

## ✨ Features

- ⏱️ **Automatic Time Tracking** - Mulai tracking otomatis saat Zed dibuka
- 💾 **Persistent Storage** - Data tersimpan antar session
- 📊 **Multi-Level Statistics** - Track current session, daily, dan all-time
- 🔔 **Periodic Updates** - Status update tiap 10 menit
- 📝 **Session History** - Simpan semua coding session
- 🎨 **Rich Logging** - ASCII art summary dengan emoji indicators

---

## 📸 Screenshots

### Session Started
```
[Tracker] Session started at 2025-10-26
[Tracker] All-time total: 45h 23m 10s
[Tracker] Today's total: 2h 15m 30s
[Tracker] 🚀 Extension initialized!
```

### Periodic Update (Every 10 minutes)
```
[Tracker] ⏰ Current: 23m 45s | Today: 2h 39m 15s | All-Time: 45h 46m 55s
```

### Session Summary (When Zed Closes)
```
╔══════════════════════════════════════════════╗
║        🎯 Coding Session Summary            ║
╠══════════════════════════════════════════════╣
║  This Session: 1h 23m 45s                   ║
║  Today Total:  3h 45m 12s                   ║
║  All-Time:     47h 10m 23s                  ║
║  Total Sessions: 28                         ║
╚══════════════════════════════════════════════╝

[Tracker] 💾 Session saved!
```

---

## 🚀 Quick Start

### Prerequisites

- **Zed Editor** - [Download](https://zed.dev/)
- **Rust toolchain** - [Install](https://rustup.rs/)
- **C++ Build Tools** (Windows only) - [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)

### Installation

#### 🪟 Windows

```powershell
# 1. Clone atau download project
git clone https://github.com/yourusername/zed-coding-tracker
cd zed-coding-tracker

# 2. Build extension
cargo build --release

# 3. Install ke Zed
mkdir "%APPDATA%\Zed\extensions\coding-tracker"
xcopy /E /I . "%APPDATA%\Zed\extensions\coding-tracker"

# 4. Restart Zed
```

#### 🐧 Linux / 🍎 macOS

```bash
# 1. Clone atau download project
git clone https://github.com/yourusername/zed-coding-tracker
cd zed-coding-tracker

# 2. Build extension
cargo build --release

# 3. Install ke Zed
mkdir -p ~/.config/zed/extensions/coding-tracker
cp -r . ~/.config/zed/extensions/coding-tracker/

# 4. Restart Zed
```

---

## 📁 Project Structure

```
zed-coding-tracker/
├── Cargo.toml              # Rust package configuration
├── extension.toml          # Zed extension manifest
├── README.md               # This file
├── src/
│   └── lib.rs             # Main extension code
└── target/
    └── release/
        └── zed_coding_tracker.{dll|so|dylib}
```

---

## 🔧 Configuration

### Mengubah Interval Update

Edit `src/lib.rs` line 187:

```rust
thread::sleep(Duration::from_secs(600)); // 600 = 10 menit
```

**Opsi interval:**
- `300` - 5 menit
- `600` - 10 menit (default)
- `900` - 15 menit
- `1800` - 30 menit

Rebuild setelah edit:
```bash
cargo build --release
```

### Mengubah Lokasi Data File

Edit `src/lib.rs` di fungsi `Storage::new()`:

```rust
// Default: ~/.config/zed/coding-tracker-data.txt
data_path.push("coding-tracker-data.txt");

// Custom:
data_path.push("my-custom-name.txt");
```

---

## 📊 Data Storage

### Lokasi File

| Platform | Path |
|----------|------|
| **Windows** | `%APPDATA%\Zed\coding-tracker-data.txt` |
| **Linux** | `~/.config/zed/coding-tracker-data.txt` |
| **macOS** | `~/.config/zed/coding-tracker-data.txt` |

### Format Data

File menggunakan format text sederhana:

```
TOTAL_SECONDS=163390
SESSION|2025-10-26|1729852800|1729857225|4425
SESSION|2025-10-25|1729766400|1729770000|3600
SESSION|2025-10-24|1729680000|1729687200|7200
```

**Format:** `SESSION|DATE|START_TIMESTAMP|END_TIMESTAMP|DURATION_SECONDS`

---

## 🐛 Troubleshooting

### Extension Tidak Muncul

**Windows:**
```powershell
# Cek extension folder
dir "%APPDATA%\Zed\extensions\coding-tracker"

# Cek DLL
dir "%APPDATA%\Zed\extensions\coding-tracker\target\release\*.dll"
```

**Linux/macOS:**
```bash
# Cek extension folder
ls -la ~/.config/zed/extensions/coding-tracker

# Cek shared library
ls -la ~/.config/zed/extensions/coding-tracker/target/release/*.{so,dylib}
```

**Solusi:**
1. Pastikan `extension.toml` ada di root folder
2. Rebuild: `cargo clean && cargo build --release`
3. Restart Zed sepenuhnya
4. Cek Zed Developer Console (`Ctrl+Shift+I` / `Cmd+Option+I`)

---

### Build Error: "linker not found"

**Windows:**
- Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)
- Pilih "Desktop development with C++"

**Linux:**
```bash
# Ubuntu/Debian
sudo apt install build-essential

# Fedora
sudo dnf install gcc

# Arch
sudo pacman -S base-devel
```

**macOS:**
```bash
xcode-select --install
```

---

### Data Tidak Tersimpan

**Cek permission:**

Windows:
```powershell
echo test > "%APPDATA%\Zed\test.txt"
```

Linux/macOS:
```bash
touch ~/.config/zed/test.txt
```

**Solusi:**
- Pastikan folder `~/.config/zed/` atau `%APPDATA%\Zed\` ada
- Cek write permission
- Jalankan Zed dengan user permission yang benar (jangan root)

---

### Log Tidak Muncul

1. Buka Zed Developer Console:
   - Windows/Linux: `Ctrl + Shift + I`
   - macOS: `Cmd + Option + I`

2. Cek tab "Console"

3. Filter dengan keyword: `[Tracker]`

4. Kalau masih kosong, cek log file:
   - Windows: `%APPDATA%\Zed\logs\`
   - Linux/macOS: `~/.config/zed/logs/`

---

## 🤝 Contributing

Kontribusi welcome! Silakan:

1. Fork repo ini
2. Buat branch baru (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push ke branch (`git push origin feature/amazing-feature`)
5. Buat Pull Request

---

## 📝 Development

### Run Tests

```bash
cargo test
```

### Check Code Quality

```bash
# Format code
cargo fmt

# Lint
cargo clippy
```

### Build for All Platforms

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Specific target
cargo build --release --target x86_64-pc-windows-msvc
```

---

## 🔐 Privacy & Security

- ✅ **Semua data disimpan lokal** - Tidak ada data yang dikirim ke server
- ✅ **No telemetry** - Extension tidak tracking apapun selain waktu coding
- ✅ **Open source** - Kode bisa diaudit siapa saja
- ✅ **Simple data format** - Text file, mudah di-backup atau dihapus

---

## 📦 Uninstall

### Windows
```powershell
Remove-Item -Recurse -Force "%APPDATA%\Zed\extensions\coding-tracker"
Remove-Item "%APPDATA%\Zed\coding-tracker-data.txt"
```

### Linux/macOS
```bash
rm -rf ~/.config/zed/extensions/coding-tracker
rm ~/.config/zed/coding-tracker-data.txt
```

Restart Zed setelah uninstall.

---

## 📄 License

MIT License - feel free to use and modify!

---

## 👤 Author

**Your Name**
- GitHub: [@Wicayonima-Reborn](https://github.com/Wicayonima-Reborn)
- Email: wicaksonodeveloper@gmail.com

---

## 🙏 Acknowledgments

- [Zed Editor](https://zed.dev/) - Amazing code editor
- [zed_extension_api](https://crates.io/crates/zed_extension_api) - Extension API
- Rust Community - For awesome tooling

---

## 📞 Support

Kalau ada issue atau pertanyaan:

1. 🐛 [Open an issue](https://github.com/Wicayonima-Reborn/zed-coding-tracker/issues)
2. 💬 [Discussions](https://github.com/Wicayonima-Reborn/zed-coding-tracker/discussions)
3. 📧 Email: wicaksonodeveloper@gmail.com

---

## ⭐ Star History

Kalau extension ini berguna, jangan lupa kasih star! ⭐

---

**Made with ❤️ and Rust 🦀**
