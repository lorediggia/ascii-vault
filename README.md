# ◈ Ascii-Vault

> A minimal, high-performance Rust TUI to vault ASCII art and dynamically inject terminal logos.

---

### ❯ Installation 

#### 1. Via Cargo (Recommended)
```
cargo install --git https://github.com/lorediggia/ascii-vault.git
```
#### 2. Via Curl Script (Automated)
```
curl -sSL https://raw.githubusercontent.com/lorediggia/ascii-vault/main/install.sh | bash
```
#### 3. Manual Binary
Download from [Releases](https://github.com/lorediggia/ascii-vault/releases):
```
chmod +x ascii-vault
mkdir -p ~/.local/bin
mv ascii-vault ~/.local/bin/
```
---

### ❯ Keybindings

| Action | Key | Effect |
| :--- | :--- | :--- |
| **New Entry** | `n` | Create new buffer |
| **Paste** | `v` | Sync from System Clipboard |
| **Copy** | `c` | Push to Wayland Clipboard |
| **Edit** | `e` | Enter Buffer modification mode |
| **Rename** | `r` | Update entry metadata |
| **Delete** | `d` | Remove with confirmation |
| **Set Logo** | `i` | Inject into Fastfetch source |
| **Reorder** | `Shift + ↑/↓` | Change item sequence |
| **Quit** | `q` | Exit application |

---

### ❯ File Structure
```
$HOME
└─ .config
   └─ ascii-vault
      ├─ config.json   ── Runtime preferences
      ├─ library.json  ── Vault database 
      └─ logo.txt      ── Fastfetch source file
```
---

### ❯ Fastfetch Integration
Update your `config.jsonc` to point to the vault output:
```
"logo": {
    "source": "~/.config/ascii-vault/logo.txt"
}
```
