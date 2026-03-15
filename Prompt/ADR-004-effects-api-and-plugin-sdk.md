# ADR-004: Effect API and Plugin SDK (AE-like, Clean-Room)

- Status: Accepted
- Date: 2026-02-16
- Owners: Architecture
- Depends on: ADR-001, ADR-002, ADR-003

## Context
Для достижения почти полного функционального покрытия AE-класса нужно:
- быстро наращивать встроенные эффекты,
- открыть внешний SDK для сообщества/студий,
- сохранить стабильность, производительность и безопасность.

## Decision
Принять двухконтурную модель эффектов:
1. Built-in Effects API (внутренние нативные эффекты движка)
2. Plugin SDK (внешние плагины с версионированным контрактом и sandbox-политикой)

Ключевые принципы:
- Clean-room implementation only.
- Совместимость по результату/UX, не по копированию чужого кода.
- Deterministic rendering и строгий lifecycle эффекта.

## Effects architecture
Базовые сущности:
- `EffectDescriptor`
  - id, name, category, version, capabilities, parameter schema
- `EffectInstance`
  - runtime state, parameter values, keyframe bindings
- `EffectContext`
  - frame/time, resolution, color space, render quality, caches
- `EffectProcessor`
  - `prepare()`, `process()`, `teardown()`

Data flow:
- Layer stack -> effect chain -> render graph nodes -> frame output
- Invalidation: пересчет только затронутых узлов/тайлoв
- GPU first, CPU fallback при недоступности capability

## Parameter model
Типы параметров:
- scalar, vec2/vec3/vec4
- color (linear-aware)
- bool, enum
- curve/ramp
- layer/asset reference
- mask/matte input

Требования:
- Все параметры keyframe-able
- Единая система interpolation/easing
- Автоматическая сериализация в project schema

## Render contract
`process(input_frames, params, context) -> output_frame`

Контракт:
- Нет скрытых глобальных сайд-эффектов
- Четкие границы памяти и времени
- Одинаковый результат при одинаковом входе
- Поддержка ROI/tile processing для производительности

## Plugin SDK model
SDK уровни:
1. v1: CPU effect plugins
2. v2: GPU kernels/plugins (ограниченный capability set)
3. v3: генераторы/процедурные эффекты + custom UI widgets

Интерфейс SDK:
- Stable ABI boundary (через C ABI или строго версионированный FFI слой)
- Manifest-based registration:
  - plugin id, version, target sdk version
  - declared permissions/capabilities
  - supported effect types

Versioning:
- `plugin_sdk_version` semver
- Backward-compat policy: N-1 минимум
- Deprecation window с предупреждениями

## Security and sandboxing
Правила безопасности для внешних плагинов:
- Capability-based permissions (filesystem/network/process access по умолчанию запрещены)
- Crash isolation (ошибка плагина не должна валить host)
- Time/memory guardrails
- Подпись плагина/доверенные источники (roadmap)

Для expressions (v2+):
- отдельный sandbox runtime
- ограниченный API
- no arbitrary host I/O

## UX integration
- Effect Browser: категории, поиск, favorites, recently used
- Preset system: save/load/share параметров эффекта
- Effect Controls: единый UI-паттерн для всех эффектов
- Fallback UX: если плагин недоступен, показать replacement/warning path

## Performance strategy
- Frame cache + tile cache
- Multi-threaded scheduling
- SIMD/GPU acceleration where applicable
- Quality levels: Draft/Preview/Final
- Benchmark suite по каждому effect family

## Test strategy
Обязательно:
- Unit tests на параметры и lifecycle
- Golden image tests для built-in эффектов
- ABI compatibility tests для SDK
- Fuzzing manifest/parser
- Stress tests: цепочки из 20-50 эффектов

## Initial built-in effect pack (v1)
Минимальный набор:
- Transform
- Fill/Tint
- Levels
- Curves
- Exposure
- Hue/Saturation
- Blur (box/gaussian)
- Glow
- Sharpen
- Noise/Grain (basic)
- Matte/Key (basic luma/chroma)

## Legal/Compliance note
Разрешено: создавать собственные эффекты с аналогичной функциональностью и похожими пользовательскими сценариями.
Запрещено: копировать исходный код, бинарные артефакты, закрытые SDK/форматы и защищенные ассеты сторонних продуктов.

## Consequences
Плюсы:
- Быстрая эволюция функциональности через built-in + community plugins.
- Контролируемая стабильность благодаря strict API и sandbox.
- Понятный путь к AE-like parity по эффектам.

Минусы:
- Высокая инженерная стоимость ABI-стабильности и тестовой матрицы.
- Понадобится tooling для SDK и диагностики плагинов.

## Implementation plan
1. Создать `effects_core` модуль (descriptor/instance/context/processor).
2. Реализовать built-in v1 effect pack.
3. Добавить `plugin_sdk_v1` (manifest + ABI + loader + validation).
4. Подключить sandbox policy + crash isolation.
5. Собрать golden tests и performance benches.

## Acceptance criteria
- Пользователь может применять цепочки встроенных эффектов и рендерить стабильно.
- Плагин v1 подключается через manifest и проходит валидацию.
- Ошибка плагина не приводит к падению всего приложения.
- Есть regression/golden тесты для базового effect pack.
- Документирован SDK lifecycle и versioning policy.

