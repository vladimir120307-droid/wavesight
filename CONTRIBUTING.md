# Contributing to WaveSight

_English · [Русский](#вклад-по-русски)_

Thanks for considering a contribution. WaveSight is built in the open and we want the contribution process to feel approachable. This document covers how to get started, what we expect, and how we review changes.

## Ways to contribute

There is a lot more useful work than the obvious "write code":

- **Capture data**: record CSI scenes with consent and contribute them to the public dataset.
- **Run benchmarks**: run `wavesight bench` on your hardware and report numbers in Discussions.
- **Translate documentation**: EN ↔ RU parity is a hard requirement, but third languages are very welcome.
- **Write a tutorial**: blog post, YouTube video, conference talk — open a PR linking to it from `docs/community.md`.
- **Test on hardware** we don't have: ESP32-C5/C6, exotic NICs, OpenWrt routers.
- **File good bug reports**: a clear, reproducible bug is worth more than a vague feature request.
- **Improve the UI**: dashboard polish, mobile app screens.
- **Write tests**: every crate has property-based + integration tests. Coverage matters.

## Code contributions

### 1. Find an issue or open one

- Look for labels: `good-first-issue`, `help-wanted`, `bug`, `enhancement`, `hardware`.
- For non-trivial changes: open an issue first and propose your approach. We will save you wasted time.

### 2. Set up the dev environment

```bash
git clone https://github.com/vladimir120307-droid/wavesight.git
cd wavesight

# Rust toolchain
rustup toolchain install 1.78
rustup component add rustfmt clippy

# Node (for dashboard)
corepack enable
pnpm install --filter ./dashboard

# Python (for training)
python -m venv .venv && source .venv/bin/activate
pip install -e training[dev]

# Pre-commit
pip install pre-commit && pre-commit install
```

### 3. Branch & commit

- Branch name: `topic/short-description` or `fix/issue-123`.
- Commit message: conventional commits (`feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`).
- Each commit should compile and pass tests on its own; rebase if needed.

### 4. Run checks locally

```bash
# Rust
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --workspace

# Dashboard
pnpm --filter ./dashboard lint
pnpm --filter ./dashboard test

# Python
ruff check training
pytest training
```

### 5. Open a pull request

- Fill in the PR template (it's short).
- Link the issue (`Closes #123`).
- Include before/after screenshots for UI changes.
- For hardware changes: include photos and a brief test report.
- Be patient with review; we are a small team.

## Code style

- Rust: `rustfmt` default, `clippy::pedantic` allowed where reasonable.
- TypeScript: Prettier + ESLint as configured.
- Python: `ruff` + `black` (default settings).
- No emoji in code or commit messages.

## Documentation policy

- Every new feature ships with EN + RU documentation. We will not merge English-only docs for user-facing features. Internal ADRs are EN-only.
- ADRs use the [template](docs/adr/_template.md).
- API changes update OpenAPI / gRPC schemas in the same PR.

## Hardware contributions

If you are donating time on hardware we don't own:

- Document the exact part numbers and firmware versions.
- Capture a short video (30 s is enough) demonstrating the behaviour.
- Add a row to `docs/hardware/compatibility.md`.

## Review expectations

- We aim for first review within **7 days**.
- We will say "no" politely if your change conflicts with the roadmap — and we'll explain why.
- Trivial typo fixes get fast-tracked.

## Code of conduct

By participating, you agree to our [Code of Conduct](CODE_OF_CONDUCT.md). Be kind, technical and patient.

---

<a id="вклад-по-русски"></a>

# Вклад в WaveSight (по-русски)

Спасибо, что рассматриваете возможность контрибьюции. WaveSight разрабатывается в открытом режиме, и мы хотим, чтобы процесс был дружелюбным.

## Способы помочь

Полезно не только писать код:

- **Сбор данных**: запишите CSI-сцены с согласием участников и добавьте в публичный датасет.
- **Запуск бенчмарков**: запустите `wavesight bench` на своём железе и опубликуйте цифры в Discussions.
- **Перевод документации**: парность EN ↔ RU — наше требование, но переводы на третьи языки тоже приветствуются.
- **Туториалы**: блог, YouTube, конференция — откройте PR со ссылкой в `docs/community.md`.
- **Тесты на железе**, которого у нас нет: ESP32-C5/C6, экзотические NIC, OpenWrt-роутеры.
- **Хорошие баг-репорты**: воспроизводимый баг ценнее размытого feature request.
- **Полировка UI**: дашборд, мобильное приложение.
- **Написание тестов**: каждый crate имеет property-based и интеграционные тесты.

## Контрибьюция кода

### 1. Найти issue или открыть новое

- Ищите лейблы: `good-first-issue`, `help-wanted`, `bug`, `enhancement`, `hardware`.
- Для нетривиальных изменений: сначала откройте issue и опишите подход.

### 2. Настройка окружения

```bash
git clone https://github.com/vladimir120307-droid/wavesight.git
cd wavesight

rustup toolchain install 1.78
rustup component add rustfmt clippy

corepack enable
pnpm install --filter ./dashboard

python -m venv .venv && source .venv/bin/activate
pip install -e training[dev]

pip install pre-commit && pre-commit install
```

### 3. Ветки и коммиты

- Имя ветки: `topic/короткое-описание` или `fix/issue-123`.
- Коммит-месседжи: conventional commits (`feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`).
- Каждый коммит должен компилироваться и проходить тесты.

### 4. Локальные проверки

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --workspace

pnpm --filter ./dashboard lint
pnpm --filter ./dashboard test

ruff check training
pytest training
```

### 5. Открытие PR

- Заполните шаблон PR (он короткий).
- Свяжите issue (`Closes #123`).
- Для UI-изменений приложите скриншоты до/после.
- Для hardware-изменений — фото и тест-репорт.
- Будьте терпеливы с ревью.

## Стиль кода

- Rust: `rustfmt`, `clippy::pedantic` где разумно.
- TypeScript: Prettier + ESLint.
- Python: `ruff` + `black`.
- Без эмодзи в коде и коммитах.

## Политика документации

- Каждая новая фича — с документацией EN + RU. Только-английская документация для user-facing фич не мерджится. Внутренние ADR — только EN.
- ADR используют [шаблон](docs/adr/_template.md).
- API-изменения обновляют OpenAPI / gRPC схемы в том же PR.

## Code of Conduct

Участвуя, вы соглашаетесь с [Code of Conduct](CODE_OF_CONDUCT.md). Будьте добры, конкретны и терпеливы.
