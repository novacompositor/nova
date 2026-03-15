# ADR-005: Timeline Engine (Subframe Timing, Interpolation, Graph Model)

- Status: Accepted
- Date: 2026-02-16
- Owners: Architecture
- Depends on: ADR-001, ADR-002, ADR-003, ADR-004

## Context
Timeline engine — центральная подсистема AE-like композитора. Она должна обеспечивать:
- точный и предсказуемый тайминг,
- плавную анимацию с гибкой интерполяцией,
- realtime отзывчивость в UI,
- корректную базу для graph editor, playback и render queue.

## Decision
Принять модель timeline на основе:
- рационального времени (`timebase_num/timebase_den`) + subframe precision,
- property-centric анимации (channels/curves/segments),
- единого evaluation graph для preview/final render,
- инкрементального invalidation + frame/tile cache.

## Time model
Основные правила:
- Композиция имеет `fps` и `timebase`.
- Все ключи и события хранятся в рациональном времени, а не только в float.
- Поддержка subframe sampling обязательна.
- Time remap работает как отдельный анимируемый channel.

Нормализация:
- Все операции trim/stretch/remap приводятся к canonical time representation.
- Сравнение времени делается без unsafe float equality.

## Data model
Сущности:
- `Composition`
- `Layer`
- `Property`
- `AnimationChannel`
- `Keyframe`
- `InterpolationSegment`
- `Marker`

`Keyframe` fields (минимум):
- `id`, `time`, `value`
- `interp_in`, `interp_out` (linear/bezier/hold/roving)
- `ease_in`, `ease_out`
- `spatial_tangent_in/out` (для path-анимации)

## Interpolation model
Поддерживаемые режимы:
- Hold
- Linear
- Bezier (value/speed)
- Roving keyframes (phase 2)

Требования:
- Детерминированный результат для одинакового входа.
- Идентичный math-path между preview и final render.
- Curve evaluation оптимизирована для массового сэмплинга.

## Graph editor model
Graph Editor строится из engine-данных, не из UI-only состояния.

Два режима данных:
- Value Graph Dataset
- Speed Graph Dataset

Engine API обязан выдавать:
- sampled points для видимого time-range,
- ключи и bezier handles,
- min/max bounds и autoscale hints,
- selected channels metadata.

## Timeline operations
MVP операции:
- add/remove/move keyframe
- trim/split/slip/slide
- layer in/out/stagger
- parenting
- markers (layer/composition)
- snapping (time/key/marker/playhead)

Все операции проходят через command system (см. ADR-002) и поддерживают undo/redo.

## Evaluation pipeline
Порядок вычислений на кадр:
1. Resolve active layers by time window
2. Evaluate layer transforms/properties at sample time
3. Evaluate effect parameters/chains
4. Resolve mattes/masks/blend dependencies
5. Emit render graph inputs

Оптимизация:
- Dirty-region invalidation по property/layer/segment
- Partial recompute вместо full timeline recompute

## Playback behavior
Режимы:
- Realtime Preview (может пропускать кадры по policy)
- Accuracy Mode (без пропуска для валидации)

Policy dropped frames:
- В preview допустим controlled frame skipping с сохранением корректного playhead time.
- В финальном рендере frame skipping запрещен.

## Caching integration
- Frame cache (RAM): для последних/часто используемых кадров
- Disk cache: для длинных превью и тяжёлых композиций
- Cache keys учитывают:
  - composition hash
  - layer/effect state hash
  - time sample
  - quality mode

## Performance guardrails
- Scrubbing latency target: < 50-80 ms на типовых сценах.
- Interactive timeline edits не должны блокировать UI thread.
- Массовое обновление keyframes — batched command apply.

## Error handling
- Некорректные каналы/битые ссылки не должны крашить playback.
- Engine возвращает structured diagnostics и fallback behavior.

## Test strategy
Обязательно:
- Unit tests: interpolation math/time normalization
- Property-based tests: monotonic time invariants
- Integration tests: trim/stretch/remap correctness
- Golden tests: graph datasets for canonical scenes
- Performance benchmarks: scrub/playback on reference projects

## Consequences
Плюсы:
- Точный фундамент для анимации и graph editor.
- Предсказуемое поведение в preview и render.
- Масштабируемость под сложные сцены и v2-фичи.

Минусы:
- Сложнее реализация time math и invalidation.
- Требует строгой тестовой дисциплины.

## Implementation plan
1. Реализовать `timeline_core` с рациональным временем и keyframe channels.
2. Добавить interpolation engine (hold/linear/bezier).
3. Подключить command-based timeline ops + undo/redo.
4. Сделать graph datasets API для UI.
5. Встроить invalidation + cache keys + playback policies.

## Acceptance criteria
- Можно анимировать базовые свойства слоя с корректной интерполяцией.
- Graph Editor получает корректные value/speed datasets.
- Scrubbing и playback стабильны и не блокируют UI.
- Time remap/trim/stretch дают предсказуемый результат.
- Все ключевые сценарии покрыты unit/integration/perf тестами.

