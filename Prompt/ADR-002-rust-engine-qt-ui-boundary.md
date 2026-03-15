# ADR-002: Rust Engine <-> Qt/QML UI Boundary

- Status: Accepted
- Date: 2026-02-16
- Owners: Architecture
- Supersedes: none
- Depends on: ADR-001 (Qt 6 + QML UI)

## Context
Нужна устойчивая архитектура для pro-grade композитора: UI должен оставаться отзывчивым, а heavy задачи (timeline eval, render, decode/encode) должны выполняться в engine без утечек бизнес-логики в UI.

## Decision
Принять модель:
- UI: Qt 6 + QML (presentation + interaction orchestration)
- Domain/Engine: Rust (project model, timeline, render graph, cache, playback scheduler)
- Media backend: FFmpeg (decode/encode/transcode/container)
- Contract: versioned Engine API boundary (command/query/event)

## Architectural boundaries
1. QML/UI layer
- Отвечает за layout, панельную систему, hotkeys, forms, визуальное состояние.
- Не содержит логики вычисления timeline/render.

2. App Facade (bridge)
- Тонкий bridge-слой между Qt и Rust.
- Маппинг UI intents -> engine commands.
- Маппинг engine events -> UI updates.

3. Rust Engine
- Единственный источник истины по project state.
- Выполняет deterministic evaluation.
- Управляет render graph, invalidation, cache, playback clock.

4. Media I/O (FFmpeg adapter)
- Изолированный модуль `media_ffmpeg`.
- Вся работа с кодеками/контейнерами/транскодированием только через него.

## API contract model
Contract style: `Command / Query / Event`

- Commands (mutating):
  - `CreateProject`, `OpenProject`, `SaveProject`
  - `AddLayer`, `SetKeyframe`, `SetPropertyValue`
  - `ImportAsset`, `QueueRenderJob`, `StartPlayback`, `StopPlayback`

- Queries (read-only snapshots):
  - `GetProjectTree`, `GetTimelineView`, `GetLayerProperties`
  - `GetGraphEditorData`, `GetPlaybackStatus`, `GetRenderQueue`

- Events (async from engine):
  - `ProjectChanged`, `TimelineInvalidated`, `FrameReady`
  - `PlaybackStateChanged`, `RenderProgress`, `RenderFailed`
  - `MediaOfflineDetected`, `GpuFallbackActivated`

## Threading model
- UI thread: только UI и dispatch intents.
- Engine worker pool: timeline eval, caching, background tasks.
- Render thread(s): GPU/CPU render path.
- FFmpeg workers: decode/encode/transcode jobs.

Правило: никакой блокировки UI thread долгими операциями.

## State management
- Canonical state хранится в Rust engine.
- UI использует подписки на события + легковесные view-model snapshots.
- Undo/Redo реализуется в engine command history.

## Error model
- Engine возвращает structured errors: `code`, `message`, `context`, `recoverable`.
- UI показывает user-facing сообщение и action hints.
- Критические ошибки пишутся в diagnostics/crash report.

## Versioning and compatibility
- Engine API имеет semver (`engine_api_version`).
- Project format имеет отдельную schema version + migration pipeline.
- Bridge обязан проверять совместимость версий при старте.

## Performance guardrails
- Target: UI response < 50 ms на типовые действия.
- Timeline scrub: минимизация full recompute через dependency invalidation.
- Frame cache приоритетнее recompute для interactive playback.
- Fallback policy: GPU issue -> безопасный CPU path без краша с явным уведомлением.

## Security and sandboxing (forward)
- Expressions/plugins (v2+) выполняются в sandbox-контуре.
- Никакой прямой произвольной I/O из expressions в хост процесс без policy.

## Consequences
Плюсы:
- Чёткое разделение ответственности.
- Масштабируемость команды (UI и engine могут развиваться независимо).
- Контролируемая производительность и тестируемость.

Минусы:
- Дополнительные затраты на bridge и строгость API контрактов.
- Требует дисциплины при добавлении новых фич.

## Implementation plan
1. `engine_api` crate: описать команды/запросы/события + типы ошибок.
2. `app_bridge` слой: адаптер Qt <-> Rust.
3. `media_ffmpeg` модуль с унифицированными задачами ingest/export.
4. Событийная шина и snapshot queries для Timeline/Project/Viewer.
5. Интеграционные тесты контракта и нагрузочные smoke-тесты UI responsiveness.

## Acceptance criteria
- UI shell может открыть проект, импортировать медиа, добавить слой, поставить keyframes, запустить preview, отправить export job.
- Все мутации проходят через commands в engine.
- UI не блокируется при import/render.
- FFmpeg backend используется для всех codec/container/export операций.
- Есть тесты на совместимость bridge с `engine_api_version`.

