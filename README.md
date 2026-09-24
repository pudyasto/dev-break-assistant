# 🧘 DevBreak — Desktop Wellness Assistant

> **Privacy-first, local-only desk habit tracker and wellness companion for software developers.**  
> Protects your eyes (20-20-20 rule), encourages regular movement, tracks work sessions, and delivers adaptive reminders without disrupting your workflow.

---

## 🌟 Key Features

* **⏱️ Precision Idle & Activity Tracking:**
  * Native Linux idle detection via GNOME Mutter D-Bus (`org.gnome.Mutter.IdleMonitor`) on Wayland & X11.
  * Cross-platform support for macOS via CoreGraphics (`CGEventSourceSecondsSinceLastEventType`) & IOKit.
  * Automatic screen lock detection (`org.gnome.ScreenSaver` / login1).

* **🧘 Intelligent Break Engine:**
  * **20-20-20 Eye Break:** 20 seconds pause every 20 minutes to reduce digital eye strain.
  * **Short Movement Break:** 3–5 minutes stretching after 45 minutes of continuous focus.
  * **Long Rest Break:** 10–15 minutes break after 90 minutes to refresh posture and mind.
  * Support for starting breaks manually, snoozing, dismissing, and auto-detecting break completion when away from desk.

* **🧠 Adaptive Reminder Engine (Phase 10):**
  * Evaluates your daily break compliance and adjusts reminder urgency dynamically.
  * Escalates to urgent notifications with targeted guidance if breaks are repeatedly skipped.
  * Calculates a daily **Desk Habit Score (0–100)** based on active hours, breaks taken, and snooze/skip ratio.

* **🧘 Guided Stretch & Exercise Library (Phase 9):**
  * Interactive stretching module featuring Eye Palming, Neck Stretch, Wrist Extensions, Shoulder Shrugs, and Seated Twists.
  * Includes step-by-step instructions, target benefits, and a built-in countdown timer for each exercise.

* **🔒 Privacy-Preserving Foreground App Tracking (Phase 8):**
  * *Opt-in* detection of active development apps (e.g., VS Code, Terminal, Browser).
  * Never records window titles, URLs, keystrokes, screenshots, or clipboard contents.

* **🤖 Local AI Wellness Tips (Phase 11):**
  * 100% offline, privacy-safe daily wellness tips generated from your previous day's habits.
  * Seamlessly connects to local LLM engines such as **Ollama** (`llama3`, `mistral`, `phi3`, etc.). No third-party cloud APIs.

* **📊 Dashboard, Statistics & History (Phases 5 & 6):**
  * Live status display with active duration and countdown badges for upcoming breaks.
  * Comprehensive 7-day and custom-range historical charts and work session breakdowns.

* **🔔 System Tray & Native Notifications (Phase 4):**
  * System tray integration with quick pause/resume controls and status indicators.
  * Native desktop notifications for eye breaks and stretch prompts.

* **🗑️ Data Management & One-Click Reset:**
  * Built-in reset utility in Settings with confirmation dialog to purge all recorded logs, sessions, and statistics with SQLite `VACUUM` while preserving your custom preferences.

---

## 🚀 Implementation Progress (Roadmap)

All phases from the Master Specification have been implemented:

| Phase | Description | Status |
| :--- | :--- | :---: |
| **Phase 0** | **Foundation:** Tauri 2.0, Vue 3, TypeScript, Tailwind CSS, SQLite (SQLx) | ✅ Completed |
| **Phase 1** | **Ubuntu Idle Detection:** GNOME Mutter D-Bus idle monitor & fallback mechanisms | ✅ Completed |
| **Phase 2** | **Activity Session Engine:** Segment tracking, crash recovery, state persistence | ✅ Completed |
| **Phase 3** | **Break Engine:** 20-20-20, short & long break rules, countdowns, break states | ✅ Completed |
| **Phase 4** | **Notifications & System Tray:** Native OS notifications and tray menu with quick controls | ✅ Completed |
| **Phase 5** | **Dashboard & Daily Statistics:** Live timer, countdown badges, habit score, daily summaries | ✅ Completed |
| **Phase 6** | **History & Weekly Analytics:** Multi-day trend charts, session timeline, compliance stats | ✅ Completed |
| **Phase 7** | **macOS Port:** CoreGraphics idle detection & platform abstraction layer | ✅ Completed |
| **Phase 8** | **Foreground App Tracking:** Privacy-first active application tracking | ✅ Completed |
| **Phase 9** | **Interactive Stretch Library:** Visual guided stretches with interactive timers | ✅ Completed |
| **Phase 10** | **Adaptive Reminder Engine:** Urgency escalation, habit-based notification tailoring | ✅ Completed |
| **Phase 11** | **Local AI Integration:** Local LLM integration (Ollama) for offline wellness tips | ✅ Completed |
| **Extra** | **Data Management:** Safe database log reset with confirmation modal | ✅ Completed |

---

## 🏗️ Architecture

DevBreak adopts a clean, modular architecture separating the Tauri Rust backend and the modern Vue 3 frontend:

```
devbreak/
├── src/                          # Vue 3 Frontend (TypeScript + Tailwind CSS)
│   ├── components/               # Reusable UI widgets (Header, Timer, Cards, Modals)
│   ├── data/                     # Stretches database & static definitions
│   ├── pages/
│   │   ├── Dashboard.vue         # Live activity overview & AI tip banner
│   │   ├── Activity.vue          # Live session monitor, break controls & hardware sensors
│   │   ├── Statistics.vue        # Daily breakdown & compliance stats
│   │   ├── History.vue           # 7-day trend charts and session timelines
│   │   ├── Stretches.vue         # Interactive guided stretch routine with timer
│   │   └── Settings.vue          # Preferences, AI config & Data Management
│   ├── services/
│   │   └── tauri.ts              # Type-safe IPC bridge to Tauri backend commands
│   ├── stores/                   # Pinia stores (activity, settings, statistics)
│   └── types/                    # Domain TypeScript interfaces
│
└── src-tauri/                    # Rust Backend (Tauri 2 Core)
    ├── migrations/               # SQLite schema migrations (0001_initial.sql)
    └── src/
        ├── domain/               # Core business logic (activity state, break rules)
        ├── application/          # Services (activity, break, settings, tray)
        ├── infrastructure/
        │   ├── database/         # SQLx query repositories & migrations
        │   └── platform/         # OS adapters (Linux D-Bus Mutter, macOS CoreGraphics)
        ├── scheduler/            # Background Tokio monitoring loop
        ├── commands/             # Invokable Tauri commands (activity, stats, ai, settings)
        ├── state.rs              # Managed AppState (database pool, locks, platform)
        ├── errors.rs             # Error types with thiserror
        └── lib.rs                # Application bootstrap & command registration
```

---

## 🛡️ Privacy Guarantee

DevBreak is strictly **local-first** and **zero-telemetry**:

* ❌ **Never** records keystrokes or logs input text.
* ❌ **Never** takes screenshots or captures screen buffers.
* ❌ **Never** inspects clipboard contents.
* ❌ **Never** sends metrics or personal data to the cloud.
* ❌ **Never** captures browser URLs or sensitive window titles.
* 📁 All data stays inside your local SQLite database:  
  `~/.local/share/com.devbreak.app/devbreak.db` (Linux)  
  `~/Library/Application Support/com.devbreak.app/devbreak.db` (macOS)

---

## 🛠️ Prerequisites & Setup

### 1. System Dependencies

#### **Ubuntu / Debian (22.04 / 24.04):**
```bash
sudo apt-get update && sudo apt-get install -y \
  libwebkit2gtk-4.1-dev \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  pkg-config \
  build-essential \
  curl
```

#### **macOS:**
* Xcode Command Line Tools: `xcode-select --install`

### 2. Rust & Node.js

* **Rust:**
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source "$HOME/.cargo/env"
  ```
* **Node.js:** v18+ or v20+ recommended.

---

## 💻 Development & Running

1. **Clone the repository and enter directory:**
   ```bash
   cd devbreak
   ```

2. **Install frontend dependencies:**
   ```bash
   npm install
   ```

3. **Start the application in development mode:**
   ```bash
   npm run tauri dev
   ```

4. **Run TypeScript check:**
   ```bash
   npm run type-check
   ```

5. **Build production bundle:**
   ```bash
   npm run tauri build
   ```

---

## 🤖 Local AI Configuration (Ollama)

To enable personalized offline wellness tips:

1. Install [Ollama](https://ollama.ai) on your system:
   ```bash
   curl -fsSL https://ollama.com/install.sh | sh
   ```
2. Pull a lightweight model (e.g. `llama3`, `mistral`, or `phi3`):
   ```bash
   ollama run llama3
   ```
3. Open **DevBreak → Settings → AI Integration (Local Only)**:
   * Toggle **Enable AI Wellness Tips** to ON.
   * Verify Endpoint (Default: `http://localhost:11434/api/generate`).
   * Set Model Name: `llama3`.
   * Click **Save Settings**.
4. The dashboard will now automatically provide daily wellness advice tailored to your activity and habit trends!

---

## 🗑️ Resetting Data

If you wish to wipe previous activity history:
1. Navigate to **Settings**.
2. Scroll to the **Data Management** section.
3. Click **Reset Data** and confirm in the pop-up modal.
4. All activity segments, session history, reminders, and daily statistics will be permanently purged and the SQLite database vacuumed, while preserving your configuration.

---

## 🧪 Testing

* **Rust Unit Tests:**
  ```bash
  cd src-tauri
  cargo test
  ```
* **Frontend Verification:**
  ```bash
  npm run type-check
  npm run build
  ```

---

## 📄 License

MIT © [DevBreak Contributors](LICENSE)
