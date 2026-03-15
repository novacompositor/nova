# ADR-001: UI Framework for AE-like Compositor

- Status: Accepted
- Date: 2026-02-16
- Owners: Product/Architecture

## Context
Проекту нужен UI уровня профессионального DCC-приложения (как AE-класс), с Linux-first качеством, высокой производительностью и предсказуемым поведением для сложных сценариев: timeline, graph editor, viewer, dockable panels, hotkeys, HiDPI, multi-monitor.

## Decision
Выбран основной UI-стек:
- Qt 6 (Qt Quick/QML для интерфейса)
- C++/Rust backend integration
- FFmpeg как центральный media backend (decode/encode/transcode/container handling)

## Why this decision
- Наиболее зрелый и проверенный desktop-стек для сложных pro UI-паттернов.
- Сильная поддержка docking/workspaces, высокое качество визуала, стабильность на Linux/Windows/macOS.
- Хороший баланс между производительностью и скоростью разработки UI.
- Упрощает цель "миграции без боли" для пользователей AE за счет гибкой настройки интерфейса и hotkeys.

## Alternatives considered
1. Tauri/Electron
- Плюсы: быстрый старт, web stack.
- Минусы: выше риск проблем с heavy pro desktop UX и latency-sensitive workflows.

2. Flutter Desktop
- Плюсы: современный UI, кроссплатформенность.
- Минусы: больше продуктовых рисков для DCC-класса редактора с глубокой desktop-интеграцией.

3. Slint
- Плюсы: легковесность, Rust-friendly.
- Минусы: ниже зрелость для большого pro creative tool класса.

## Consequences
Позитивные:
- Высокий потолок качества UI/UX и визуальной полировки.
- Снижение архитектурного риска на ранних этапах.

Негативные:
- Более высокая сложность сборки/интеграции.
- Потребуется дисциплина по разделению UI/Engine границ.

## Implementation guardrails
- UI слой не содержит бизнес-логики рендера и таймлайна.
- Engine API — стабильный контракт между UI и core.
- Все media codec/container задачи проходят через FFmpeg backend.
- Linux-first QA matrix обязательна с первого beta-цикла.

## Acceptance criteria
- Есть рабочий UI shell: Project, Timeline, Viewer, Effects, Graph Editor, Render Queue.
- Поддержаны docking layouts, workspace presets, AE-like hotkey profile.
- Стабильная работа на Linux (Wayland/X11), Windows, macOS на базовых сценариях.

