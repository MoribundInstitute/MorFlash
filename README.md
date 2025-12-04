<p align="center">
  <img src="assets/logo/morflash-logo.png" width="200" alt="MorFlash Logo">
</p>

<h1 align="center">MorFlash</h1>

<p align="center">
  Offline. Fast. Multilingual. Fully customizable flashcard learning.
</p>

<p align="center">

  <!-- Version badge -->
  <a href="https://github.com/MoribundMurdoch/morflash/releases">
    <img src="https://img.shields.io/github/v/release/MoribundMurdoch/morflash?label=version&color=blue" alt="Version">
  </a>

  <!-- Build badge (Cargo build) -->
  <a href="https://github.com/MoribundMurdoch/morflash/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/MoribundMurdoch/morflash/rust.yml?branch=main&label=build" alt="Build Status">
  </a>

  <!-- Spec badge -->
  <a href="https://github.com/MoribundMurdoch/mflash-spec">
    <img src="https://img.shields.io/badge/.mflash-spec-blueviolet" alt="mflash spec">
  </a>

  <!-- License badge -->
  <a href="https://github.com/MoribundMurdoch/morflash/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/MoribundMurdoch/morflash?color=brightgreen" alt="License">
  </a>

</p>

---

## 📚 Table of Contents
<details>
<summary><strong>Click to expand</strong></summary>

- [🌟 Features](#-features)
  - [Flexible Study Interface](#flexible-study-interface)
  - [Rich Customization](#rich-customization)
  - [Import & Deck Support](#import--deck-support)
  - [Offline & Self-Contained](#offline--self-contained)
- [🎥 Demo](#-demo)
- [🖼️ Screenshots](#️-screenshots)
- [📘 mflash File Format](#-mflash-file-format)
- [🚀 Getting Started](#-getting-started)
- [🎮 Usage](#-usage)
- [🎨 Customization](#-customization)
- [🛠️ Building the Project](#️-building-the-project)
- [📄 License](#-license)

</details>

---

# 🌟 Features

### **Flexible Study Interface**
- Draggable, repositionable flashcards  
- Independently movable images and videos  
- Centered card layout with configurable margins  

### **Rich Customization**
| Feature               | Description |
|----------------------|-------------|
| **Backgrounds**       | Custom tiling background images |
| **Sound Effects**     | Selectable sounds for correct/incorrect answers and deck completion |
| **Fonts**             | System fonts, Public Pixel, or custom-loaded font files |
| **UI Options**        | Adjustable themes, sounds, visuals, and interaction behavior |

### **Import & Deck Support**
- Import tab-separated `.txt` files  
- Embedded media support  
- Native `.mflash` multilingual deck format  

### **Offline & Self-Contained**
- No network access required  
- No accounts, no sync, no ads  
- Fully local + fast load times  

---

# 🎥 Demo

<p align="center">
  <img src="assets/demo/morflash-demo.gif" width="640" alt="MorFlash Demo">
</p>

*A short demonstration of drag-based study mode, media movement, and answer reveal flow.*

---

# 🖼️ Screenshots

![MorFlash Study Interface](assets/screenshots/MorFlash_Screenshot.png)

The study UI supports movable cards, draggable media, and responsive layout.

---

# 📘 mflash File Format

MorFlash uses the `.mflash` deck format.

**mflash** is an open standard with:

- Multilingual per-card language metadata  
- Tags, notes, examples  
- Media attachments  
- JSON-based v1 format  
- Future ZIP container support planned  

🔗 Official specification:  
https://github.com/MoribundMurdoch/mflash-spec

---

# 🚀 Getting Started

```bash
git clone https://github.com/MoribundMurdoch/morflash
cd morflash
cargo run
