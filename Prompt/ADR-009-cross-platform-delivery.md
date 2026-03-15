# ADR-009: Cross-Platform Delivery (Linux, Windows, macOS)

- Status: Accepted
- Date: 2026-02-17
- Owners: Architecture/Release
- Depends on: ADR-001, ADR-002, ADR-006, ADR-007

## Context
Продукт должен быть Linux-first, но production-ready на всех трёх desktop-платформах:
- Linux
- Windows
- macOS

Нужна единая кодовая база без Flutter, с минимальным OS-specific кодом и стабильным release process.

## Decision
Принять модель:
- Единый core engine (Rust/C++)
- Единый UI layer (Qt/QML)
- Единый media backend (FFmpeg)
- Кроссплатформенный render abstraction (GPU-first + CPU fallback)
- Узкий platform adapter layer для OS-specific интеграций

## Repository structure (target)
- `crates/` или `core/`: engine, timeline, render graph, effects, media
- `ui_qt/`: общий UI
- `platform/linux/`: Linux adapters and packaging scripts
- `platform/windows/`: Windows adapters and installer scripts
- `platform/macos/`: macOS adapters, signing/notarization scripts
- `ci/`: matrix pipelines and release workflows
- `packaging/`: shared packaging configs/templates

## Platform abstraction boundaries
1. Shared domain (100% common)
- Project model
- Timeline/effects/render logic
- Media ingest/export orchestration

2. Platform-specific adapters (thin)
- Native file dialogs
- System paths and permissions
- Clipboard/drag-drop edge cases
- Window manager integration
- GPU diagnostics hooks

Правило:
- Любая бизнес-логика в platform layer запрещена.

## Graphics backend policy
- Linux: Vulkan (primary), CPU fallback
- Windows: DX12 or Vulkan (policy-controlled), CPU fallback
- macOS: Metal (primary), CPU fallback

Runtime modes:
- `Auto` (default)
- `GPU Preferred`
- `CPU Safe Mode`

## Media backend policy
- Все codec/container/decode/encode/transcode операции идут через FFmpeg
- Единые preset profiles across OS
- Platform-specific codec availability учитывается в diagnostics

## Build and CI matrix
Обязательная CI matrix:
- Linux (Ubuntu LTS + latest stable)
- Windows (recent stable image)
- macOS (recent stable image)

Pipeline stages:
1. Lint + unit tests
2. Integration tests
3. Render/media smoke tests
4. Packaging dry-run
5. Signed release artifacts (for release tags)

## Packaging and distribution
Linux:
- AppImage (primary)
- Flatpak (secondary)
- deb/rpm (community or later official)

Windows:
- MSI (primary)
- Optional portable zip

macOS:
- `.app` + `.dmg`
- Code signing + notarization mandatory for stable releases

## Release channels
- `nightly`: автоматические сборки с main
- `beta`: стабилизированный срез перед стабильным релизом
- `stable`: подписанные проверенные релизы

## QA strategy by platform
Минимальные regression suites:
- Startup/open/save project
- Timeline scrubbing
- Preview playback
- FFmpeg export presets
- Crash recovery path

Дополнительно:
- Linux: Wayland/X11 checks
- Windows: DPI/scaling + GPU driver variance
- macOS: Retina + sandbox/notarization behavior

## Observability and supportability
- Unified structured logs with platform tags
- Crash reporting (opt-in)
- Diagnostics panel:
  - OS/build info
  - GPU backend
  - FFmpeg availability/version
  - active fallback mode

## Risks and mitigations
1. GPU driver instability
- Mitigation: robust fallback + backend policy flags + diagnostics

2. Packaging fragmentation
- Mitigation: standardized scripts and CI release gates

3. macOS signing friction
- Mitigation: early signing pipeline setup in beta phase

4. Platform UX inconsistency
- Mitigation: shared UI components and cross-platform design QA checklist

## Implementation plan
1. Создать папки `platform/linux`, `platform/windows`, `platform/macos` + scaffolding docs.
2. Ввести platform adapter interfaces в app_bridge.
3. Настроить CI matrix для трех ОС (lint/test/smoke).
4. Добавить packaging templates для AppImage/MSI/DMG.
5. Запустить nightly channel с артефактами.

## Acceptance criteria
- Проект собирается и проходит базовые тесты на Linux/Windows/macOS.
- Выпускаются nightly артефакты для всех трех ОС.
- Export presets через FFmpeg работают на каждой платформе.
- Есть fallback policy и diagnostics для GPU проблем.
- Stable release process покрывает signing/notarization requirements.

