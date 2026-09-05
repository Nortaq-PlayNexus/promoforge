# PROMOFORGE :: THE PRESS MACHINE

**Turn a local codebase into a launch-ready marketing strategy — press kits, store metadata, social content, campaign plans.** Generated from an actual analysis of your code on disk, not guesses.

<p align="center">
  <img src="https://img.shields.io/badge/TAURI-2-ffc430?style=flat-square&logo=tauri&logoColor=ffc430&labelColor=0a0e1a" alt="tauri"/>
  <img src="https://img.shields.io/badge/REACT-19-3dd5ff?style=flat-square&logo=react&logoColor=3dd5ff&labelColor=0a0e1a" alt="react"/>
  <img src="https://img.shields.io/badge/RUST-multi%2Dcrate-B8FF1E?style=flat-square&logo=rust&logoColor=B8FF1E&labelColor=0a0e1a" alt="rust"/>
  <img src="https://img.shields.io/badge/STAGE-0.1.0-E8E8E8?style=flat-square&labelColor=0a0e1a" alt="stage"/>
  <a href="LICENSE"><img src="https://img.shields.io/badge/LICENSE-MIT-ff3b3b?style=flat-square&labelColor=0a0e1a" alt="license"/></a>
</p>

```
[ PRESS ]  your codebase in, ten deliverables out.
```

<pre>
IDENT ......... PRESS-01
CLASS ......... LAUNCH-STRATEGY FORGE
STATUS ........ EARLY / PIPELINE WIRED
STACK ......... TAURI 2 + REACT + MULTI-CRATE RUST
INPUT ......... A LOCAL CODEBASE
LINK .......... /promoforge
</pre>

---

## // 01 :: SIGNAL

Instead of asking you to describe your product, PromoForge reads the project on disk: it scans the filesystem, classifies the project type, infers features and target personas, and builds a **Product Knowledge Base (PKB)** that drives every downstream deliverable — press kits, store metadata, social posts, and campaign plans.

---

## // 02 :: THE FORGE (HOW IT RUNS)

```
┌────────────────────────────────────────────────────────────┐
│  YOUR PROJECT FOLDER                                        │
└──────────────────┬─────────────────────────────────────────┘
                   │ 1. filesystem scan
                   ▼
        ┌───────────────┐   2. classify    4. build PKB
        │  FILESYSTEM   │──────────────►┌──────────────────────┐
        │  WALKER       │                │  PRODUCT KNOWLEDGE   │
        └───────┬───────┘                │  BASE (features,     │
                │ 3. analyze             │  personas, tech,     │
                │                        │  competitors, risks) │
                ▼                        └──────────┬───────────┘
        ┌───────────────┐                           │ 5. strategy
        │  ANALYZER     │◄─────────────────────────►│   engine
        └───────────────┘                           │  (playbooks)
                                                   │
                                                   ▼
                                     ┌──────────────────────────┐
                                     │  CAMPAIGN + STRATEGY      │
                                     │  (phases, assets, KPIs,   │
                                     │   budget, risks)          │
                                     └──────────────┬───────────┘
                                                    │ 6. content gen (LLM)
                                                    ▼
                                     ┌──────────────────────────┐
                                     │  TEXT ASSETS: press kits, │
                                     │  store metadata, social   │
                                     └──────────────────────────┘
```

The app is split into focused Rust crates under `src-tauri/crates/`:

| Crate | Role |
|-------|------|
| `core` | shared domain types (Project, PKB, Strategy, Campaign) |
| `storage` | SQLite + project persistence |
| `ingestion` | filesystem walker, project classifier, analyzer |
| `pkb` | Product Knowledge Base builder + validator |
| `strategy` | playbook selection and campaign/strategy generation |
| `api` | LLM integration (Anthropic / OpenAI) for content generation |

The React frontend (`src/`) provides pages for projects, analysis, strategy view, asset approval, and settings.

---

## // 03 :: SETUP // BOOT THE PRESS

```bash
$ npm install
$ npm run tauri dev
```

> On first use you'll add your own API key (Anthropic or OpenAI) in Settings. Keys are stored locally and never committed.

---

## // 04 :: LAYOUT

```
promoforge/
├── src/                  React + TypeScript frontend (Vite)
├── src-tauri/
│   ├── src/              Tauri command layer
│   ├── bins/             (main/lib entry points)
│   └── crates/
│       ├── core/         shared domain model
│       ├── storage/      persistence + database
│       ├── ingestion/    codebase scan & analysis
│       ├── pkb/          knowledge base build/validate
│       ├── strategy/     campaign + strategy generation
│       └── api/          LLM content generation
├── package.json
└── vite.config.ts
```

---

## // 05 :: STATUS // MANIFEST

Early-stage (`0.1.0`). Core pipeline (scan → analyze → strategy → content) is wired end-to-end. Strategy generation, budgets, and playbook-driven planning are functional; content output quality depends on the configured LLM.

<details>
  <summary><code>$ cat manifest/pipeline</code></summary>

1. Filesystem scan → 2. classify → 3. analyze → 4. build PKB → 5. strategy engine (playbooks) → 6. content gen (LLM)

</details>

---

## // 06 :: LEGAL

**License:** [MIT](LICENSE)

---

```
 ┌─────────────────────────────────────────────┐
 │  PRESS MACHINE // CODEBASE IN, CAMPAIGN OUT │
 │  PROMOFORGE-01 // SIGNAL LOCKED            │
 └─────────────────────────────────────────────┘
END OF TRANSMISSION
```