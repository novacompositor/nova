# ADR-006: Render Graph and GPU/CPU Fallback

- Status: Accepted
- Date: 2026-02-16
- Owners: Architecture
- Depends on: ADR-001, ADR-002, ADR-004, ADR-005

## Context
Для AE-like композитора критично иметь:
- высокопроизводительный realtime preview,
- детерминированный финальный рендер,
- устойчивость к проблемам драйверов/GPU на Linux,
- прозрачный fallback путь на CPU без потери проекта.

## Decision
Принять render architecture:
- Node-based Render Graph в engine
- GPU-first execution path (wgpu backend)
- CPU fallback path для всех критичных операций
- Unified frame contract для preview и final render

## Core render principles
1. Determinism
- При одинаковых входах и настройках финальный кадр детерминирован.
- Preview и final используют один pipeline semantics (разница только quality policy).

2. Separation of concerns
- UI не содержит render logic.
- Render Graph строится в engine из timeline/effects state.

3. Fault tolerance
- GPU failures не должны приводить к потере сессии.
- Автоматический graceful fallback на CPU с уведомлением в UI.

## Render graph model
Сущности:
- `RenderNode` (input/output contracts)
- `RenderEdge` (data dependency)
- `RenderPass` (execution unit)
- `RenderTarget` (texture/buffer/frame)
- `ExecutionPlan` (topological ordered passes)

Типы узлов:
- Source nodes (media/text/solid)
- Transform nodes
- Effect nodes
- Matte/Mask nodes
- Blend/Composite nodes
- Output nodes

## Build and invalidation
Graph build:
- На основе активных слоев, эффектов, масок, тайм-сэмпла.
- Topological sort + cycle detection.

Invalidation:
- Dirty-set по layer/property/effect/asset/time.
- Rebuild only affected subgraph.
- Partial re-execution по ROI/tile если поддерживается узлами.

## Execution backends
1. GPU backend (default)
- wgpu abstraction
- Backend targets: Vulkan (Linux/Windows), Metal (macOS), DX12 (Windows)
- Shader pipeline cache для ускорения warm runs

2. CPU backend (fallback)
- Reference implementation для core operations
- Используется при:
  - отсутствии capability,
  - device lost,
  - драйверных сбоях,
  - диагностическом safe mode.

## Capability detection and policy
Startup probe:
- GPU adapter, driver info, supported features/limits
- Проверка critical capabilities для рендера

Runtime policy:
- `Auto`: GPU where safe, CPU fallback on failure
- `GPU Preferred`
- `CPU Safe Mode`

UI diagnostics:
- Показ active backend, fallback reason, recommended actions

## Frame contract
`render_frame(composition_id, sample_time, quality_mode) -> FrameResult`

`FrameResult` содержит:
- pixel buffer/texture handle
- color metadata
- timing metadata
- diagnostics (warnings/fallback flags)

## Color and alpha rules
- Internal linear workflow (MVP: linear/sRGB)
- Явные правила premultiplied/unpremultiplied alpha
- Конвертация color spaces на границах ingest/output

## Memory and cache strategy
- Render target pool reuse
- Frame cache (RAM)
- Tile/Intermediate cache (optional phase)
- Budget-based eviction (LRU + priority by playhead vicinity)

## Playback and render queue interaction
- Preview jobs: latency-priority scheduler
- Final render jobs: throughput-priority scheduler
- Isolation: final render queue не должен ломать интерактивность UI

## Error model
Категории ошибок:
- Recoverable GPU error -> fallback + warning
- Recoverable asset/effect error -> placeholder + log
- Non-recoverable internal error -> abort current frame + detailed report

Логи:
- structured diagnostics с correlation id кадра/задачи

## Test strategy
Обязательно:
- Graph correctness tests (topology, cycles, dependencies)
- Golden frame tests (GPU vs CPU tolerance thresholds)
- Device-loss simulation tests
- Stress tests: длинные компы, heavy effects chains
- Performance benchmarks: 1080p/4K preview and final render

## Performance guardrails
- Preview latency target: интерактивный отклик для timeline scrubbing
- Stable FPS goals на референсных сценах
- Ограничение VRAM/RAM бюджета для предотвращения OOM

## Consequences
Плюсы:
- Стабильная архитектура для realtime и final render.
- Контролируемая деградация качества/скорости без падений.
- Прозрачная диагностика проблем на Linux GPU ecosystem.

Минусы:
- Высокая стоимость поддержки dual backend.
- Нужна строгая калибровка golden tests и tolerance.

## Implementation plan
1. Реализовать `render_graph_core` (nodes/edges/execution plan).
2. Подключить GPU backend (wgpu) для core passes.
3. Реализовать CPU reference backend для критичных passes.
4. Добавить capability probe + auto fallback policy.
5. Внедрить diagnostics panel и regression/perf test suites.

## Acceptance criteria
- Preview работает на GPU и корректно fallback-ится на CPU при сбое.
- Final render детерминирован и не зависит от UI состояния.
- Graph invalidation минимизирует лишние пересчеты.
- Есть тесты на device loss, golden frames и performance budgets.
- Пользователь получает понятные сообщения о backend status и ошибках.

