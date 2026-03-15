# ADR-007: Media Pipeline (FFmpeg Ingest, Proxy, Decode, Export)

- Status: Accepted
- Date: 2026-02-16
- Owners: Architecture
- Depends on: ADR-002, ADR-003, ADR-006

## Context
Для AE-like композитора media pipeline должен быть:
- кроссплатформенным и предсказуемым,
- устойчивым к проблемным исходникам,
- производительным для realtime preview,
- единым для импорт/прокси/экспорт.

## Decision
Центральный media backend: `FFmpeg`.

Все задачи, связанные с:
- codec decoding/encoding,
- container demux/mux,
- transcoding,
- proxy generation,
- audio/video stream handling,
выполняются через `media_ffmpeg` модуль.

## Pipeline overview
1. Ingest
- Detect container/streams/metadata
- Normalize media info (fps, timebase, color, audio layout)
- Build canonical asset descriptor

2. Decode
- Packet -> frame pipeline
- Accurate seek + pre-roll policy
- Sync against composition timeline timebase

3. Proxy
- Optional ingest-time or background proxy generation
- Profile-based proxies (edit-friendly)
- Transparent swap original/proxy in playback

4. Export
- Rendered frames/audio -> FFmpeg mux/encode
- Presets for web/editing/sequence workflows
- Deterministic output settings per preset

## Canonical asset model
Каждый импортированный asset получает:
- `asset_id`
- `source_uri`
- `stream_map` (video/audio/subs)
- `technical_metadata` (duration, fps, timebase, resolution, pixel format, sample rate)
- `color_metadata` (primaries/transfer/matrix if present)
- `proxy_state`

## Time and sync policy
- Internal engine time: рациональное время + subframe.
- Stream timestamps из FFmpeg нормализуются к engine timebase.
- A/V sync проверяется на ingest и при playback.
- При VFR источниках сохраняется корректная timestamp-driven модель, не наивный CFR assumptions.

## Decode strategy
- Threaded decode workers
- Read-ahead window around playhead
- Smart seek:
  - keyframe seek + decode-to-target
  - cache nearest decoded ranges
- Frame drop policy допускается только в preview режиме

## Proxy strategy
Proxy profiles (MVP):
- `Proxy-Low` (быстрый монтаж/скраб)
- `Proxy-Mid` (баланс)
- `Proxy-High` (почти финал)

Требования:
- Proxy хранится как derivation, оригинал неизменен
- Возможность relink/regen proxies
- UI всегда показывает active source mode (Original/Proxy)

## Export strategy
Базовые output presets:
- `Web H.264`
- `Editing ProRes`
- `Image Sequence PNG`
- `Image Sequence EXR`
- `Alpha-safe preset` (где поддерживается)

Правила:
- Экспорт всегда через FFmpeg pipeline
- Preset фиксирует codec/container/pixel format/audio settings
- Advanced panel разрешает ручную настройку, но с validation

## Error handling and resilience
Классы ошибок:
- Unsupported codec/container -> recoverable with clear UX
- Corrupted stream packets -> partial decode with diagnostics
- Export failure -> job retry options + detailed log

Fallbacks:
- Suggest proxy transcode for problematic media
- Safe decode mode for unstable files

## Performance and cache
- Decoded frame cache (RAM)
- Optional disk cache for heavy assets
- Budgeted eviction strategy
- Background prefetch around playhead

## Color handling
- Preserve source color metadata where available
- Normalize into engine working space (MVP linear/sRGB path)
- Output conversion policy controlled by export preset

## API contract (high-level)
Commands:
- `ImportAsset(uri)`
- `CreateProxy(asset_id, profile)`
- `SetAssetSourceMode(asset_id, original|proxy)`
- `QueueExport(job_spec)`

Queries:
- `GetAssetMetadata(asset_id)`
- `GetProxyStatus(asset_id)`
- `GetExportJobStatus(job_id)`

Events:
- `AssetImported`
- `ProxyProgress`
- `ExportProgress`
- `ExportFailed`
- `MediaDecodeWarning`

## Test strategy
Обязательно:
- Ingest compatibility matrix tests (containers/codecs)
- Decode correctness tests (seek/sync/frame accuracy)
- Export golden tests (preset consistency)
- Fuzzing for demux/parser boundaries
- Long-run stability tests on Linux GPU/CPU combinations

## Legal/compliance note
Media pipeline строится на открытых и легально интегрируемых компонентах.
Совместимость достигается через собственную реализацию и открытые documented workflows, без копирования проприетарных решений.

## Consequences
Плюсы:
- Единая, предсказуемая media-архитектура.
- Быстрый путь к production-экспорту и proxy workflow.
- Снижение интеграционных рисков.

Минусы:
- Высокая сложность edge-case поддержки медиафайлов.
- Большая тестовая матрица по кодекам/контейнерам.

## Implementation plan
1. Создать `media_ffmpeg` module с ingest/decode/export services.
2. Ввести canonical asset descriptor и stream normalization.
3. Реализовать proxy profiles + background generation.
4. Подключить export presets + advanced validation.
5. Добавить test matrix + diagnostics logging.

## Acceptance criteria
- Импорт популярных форматов стабилен и метаданные корректны.
- Playback поддерживает sync и seek на типовых production файлах.
- Proxy workflow работает прозрачно и ускоряет preview.
- Экспорт через presets детерминирован и повторяем.
- Ошибки media pipeline диагностируются без падения приложения.

