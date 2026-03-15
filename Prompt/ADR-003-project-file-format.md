# ADR-003: Project File Format, Versioning, Migrations, Autosave/Recovery

- Status: Accepted
- Date: 2026-02-16
- Owners: Architecture
- Depends on: ADR-001, ADR-002

## Context
Нужен формат проекта, который:
- стабильный и предсказуемый для продакшна,
- поддерживает долгую эволюцию схемы без поломок,
- безопасен при крэшах,
- удобен для диагностики и автоматических миграций,
- не привязан к проприетарным форматам.

## Decision
Выбран двухуровневый формат:
1. `*.aefork.json` (canonical project document, человекочитаемый)
2. `*.aefork.bundle/` (опциональный пакет с прокси/кэшем/метаданными)

Версионирование:
- `project_schema_version` (integer, mandatory)
- `engine_api_version` (semver string, informative)

Сериализация:
- JSON для v0/v1 (простая отладка, git-friendly)
- Возможность перехода на бинарный слой (MsgPack/CBOR) для performance-only snapshots в будущем

## File structure (high-level)
Top-level fields:
- `project_id`
- `project_schema_version`
- `created_at`, `updated_at`
- `settings` (fps, resolution, color, audio)
- `assets` (media references + metadata)
- `compositions`
- `render_queue_presets`
- `ui_state` (workspace/hotkeys profile reference)
- `extensions` (future-safe namespace)

## Asset reference policy
- Проект хранит URI-ссылки на ассеты, не встраивает оригинальные медиа по умолчанию.
- Поддержка относительных путей при сохранении рядом с проектом.
- Релинк-механизм с fingerprint (size/hash/duration/fps/timebase).
- Прокси-медиа хранится в bundle/cache зоне с явной пометкой derivation.

## Determinism requirements
- Timeline и property evaluation не зависят от порядка сериализации JSON.
- Для float-полей сохраняются нормализованные precision rules.
- Все тайминги в рациональной форме: `timebase_num/timebase_den` + subframe.

## Migrations strategy
- Каждая версия схемы имеет явный migrator: `vN -> vN+1`.
- При открытии проекта выполняется цепочка миграций до текущей версии.
- Миграции идемпотентны и покрыты regression tests.
- При критической миграции сохраняется backup `*.pre_migration.bak`.

## Autosave and recovery
Autosave:
- Интервал по умолчанию: 60 сек (настраиваемый).
- Сохраняются incremental snapshots + rolling window (например, последние 30).

Recovery:
- При старте проверяется crash marker.
- Если найдены несохраненные snapshots, UI предлагает recovery wizard:
  - `restore latest`,
  - `restore selected`,
  - `discard`.

Atomic write policy:
- Сохранение через temp file + fsync + atomic rename.
- Никогда не перезаписывать основной файл напрямую.

## Validation and integrity
- JSON schema validation при open/save.
- Semantic validation (уникальность id, валидные ссылки, корректные time ranges).
- Ошибки делятся на recoverable/non-recoverable с понятным UX.

## Interchange policy
- Экспорт/импорт через открытые и документированные форматы (например OTIO where applicable).
- Не обещать бинарную совместимость с закрытыми проприетарными форматами.

## Security
- Проектный файл рассматривается как untrusted input.
- Parser hardening + fuzzing.
- Ограничения на размеры/глубину/ресурсопотребление при загрузке.

## Consequences
Плюсы:
- Прозрачная отладка и контроль эволюции формата.
- Низкий риск потери данных благодаря autosave + atomic writes.
- Хорошая база для командной и CI-валидации проектов.

Минусы:
- JSON тяжелее бинарных форматов на очень больших проектах.
- Нужна дисциплина миграций при каждом schema change.

## Legal/Compliance note
Формат проекта, структура полей и поведение должны быть оригинальной реализацией команды.
Допустимо повторять пользовательские сценарии и функциональные возможности уровня AE-like, но недопустимо копировать чужой исходный код, закрытые форматы и защищенные материалы.

## Implementation plan
1. Создать `project_schema` crate/module с typed model.
2. Добавить `schema_version` и миграционный раннер.
3. Реализовать atomic save + autosave snapshots.
4. Добавить recovery wizard API для UI.
5. Подключить validation + fuzz/integration tests.

## Acceptance criteria
- Новый проект создается/сохраняется/открывается без потери данных.
- Проект старой версии корректно мигрирует до текущей.
- После искусственного крэша recovery восстанавливает последние изменения.
- Невалидные файлы корректно диагностируются и не приводят к падению.

