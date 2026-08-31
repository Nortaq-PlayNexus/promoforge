# PromoForge AI

A desktop app (Tauri 2 + React + Rust) that turns a local software project into a launch-ready **marketing strategy** — press kits, store metadata, social content and campaign plans — generated from an actual analysis of your codebase.

Instead of asking you to describe your product, PromoForge reads the project on disk: it scans the filesystem, classifies the project type, infers features and target personas, and builds a **Product Knowledge Base (PKB)** that drives every downstream deliverable.

## How it works

```
┌─────────────────────────────────────────────────────────────────────┐
│  Your project folder                                                 │
└──────────────┬──────────────────────────────────────────────────────┘
               │ 1. Filesystem scan
               ▼
        ┌───────────────┐     2. Classify       4. Build PKB
        │  Filesystem   │──────────────────►┌──────────────────────┐
        │  Walker       │                    │  Product Knowledge   │
        └───────┬───────┘                    │  Base (features,     │
                │ 3. Analyze                 │  personas, tech,     │
                │                            │  competitors, risks) │
                ▼                            └──────────┬───────────┘
        ┌───────────────┐                               │ 5. Strategy engine
        │  Analyzer     │◄─────────────────────────────►┘ (playbooks)
        └───────────────┘
                                                        │
                                                        ▼
                                          ┌──────────────────────────┐
                                          │  Campaign + Strategy      │
                                          │  (phases, assets, KPIs,   │
                                          │   budget, risks)          │
                                          └────────────┬─────────────┘
                                                       │ 6. Content gen (LLM)
                                                       ▼
                                          ┌──────────────────────────┐
                                          │  Text assets: press      │
                                          │  kits, store metadata,   │
                                          │  social posts            │
                                          └──────────────────────────┘
```

The app is split into focused Rust crates under `src-tauri/crates/`:

| Crate | Role |
|-------|------|
| `core` | Shared domain types (Project, PKB, Strategy, Campaign) |
| `storage` | SQLite + project persistence |
| `ingestion` | Filesystem walker, project classifier, analyzer |
| `pkb` | Product Knowledge Base builder + validator |
| `strategy` | Playbook selection and campaign/strategy generation |
| `api` | LLM integration (Anthropic / OpenAI) for content generation |

The React frontend (`src/`) provides pages for projects, analysis strategy view, asset approval and settings.

## Getting started

```bash
npm install
npm run tauri dev
```

> On first use you'll add your own API key (Anthropic or OpenAI) in Settings. Keys are stored locally and never committed.

## Project structure

```
promoforge/
├── src/                  React + TypeScript frontend (Vite)
├── src-tauri/
│   ├── src/              Tauri command layer
│   ├── bins/            (main/lib entry points)
│   └── crates/
│       ├── core/         Shared domain model
│       ├── storage/      Persistence + database
│       ├── ingestion/    Codebase scan & analysis
│       ├── pkb/          Knowledge base build/validate
│       ├── strategy/     Campaign + strategy generation
│       └── api/          LLM content generation
├── package.json
└── vite.config.ts
```

## Status

Early-stage (`0.1.0`). Core pipeline (scan → analyze → strategy → content) is wired end-to-end. Strategy generation, budgets, and playbook-driven planning are functional; content output quality depends on the configured LLM.
