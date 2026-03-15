# UI Blueprint v0.1 (AE-like Compositor, Linux-first)

## 1. Product UX goal
Цель интерфейса: дать пользователю AE знакомую ментальную модель и более современную, быструю и стабильную работу на Linux.

## 2. Primary layout (default workspace)
Панели по умолчанию:
- Left: Project + Media Bin + Effects Presets
- Center: Composition Viewer
- Bottom: Timeline
- Right: Effect Controls + Properties + Align/Info
- Secondary tab: Graph Editor
- Bottom-right: Render Queue

Поведение:
- Все панели dockable, detachable, сохраняются в workspace preset.
- Быстрое переключение `Workspaces`: `AE Familiar`, `Animation`, `Compositing`, `Color`, `Text`.

## 3. Core UI modules
1. App Shell
- Top menu, command palette, workspace switcher, status/perf bar.

2. Project Panel
- Folder/tag search, bins, smart collections, proxy status, relink workflow.

3. Timeline Panel
- Layer stack, in/out, stretch, markers, parenting, shy/solo/lock, snapping.
- Режимы отображения keyframes и mini-graph.

4. Composition Viewer
- Zoom/pan/fit controls, overlays, safe margins, guides/rulers, motion path handles.

5. Effect Controls
- Param groups, keyframe buttons, reset, compare, favorite params.

6. Graph Editor
- Value/Speed graphs, bezier handles, presets, keyframe stats.

7. Render Queue
- Presets, output module, codec/container (через FFmpeg), batch jobs, error logs.

## 4. Visual direction
- Стиль: Pro dark-neutral + high-contrast accents (без кислотных цветов).
- Типографика: читаемая, плотная, desktop-first, с четкой иерархией.
- Density profiles: `Comfortable` / `Compact` (по умолчанию Compact).
- Иконки: геометрические, минималистичные, единая сетка.

## 5. Interaction design rules
- Target latency для UI actions: визуальный отклик < 50 ms.
- Undo/Redo: глобально консистентное, с понятными action labels.
- Важные операции подтверждать только при реальном риске потери данных.
- Все длительные задачи показывают progress + возможность background mode.

## 6. AE migration design
- Hotkey Profile: `AE-like` (по умолчанию для migration mode).
- Workspace: `AE Familiar` предустановлен.
- Термины в UI совместимы с привычным словарем motion-дизайнера.
- Встроенные onboarding сценарии: "как сделать X из AE".

## 7. Accessibility and internationalization
- Масштаб UI 100-200%, корректный HiDPI/fractional scaling.
- Keyboard-first navigation.
- High contrast mode.
- Базовая локализация UI-строк без ломки layout.

## 8. Linux-first technical UX constraints
- Проверка в Wayland и X11.
- Корректные drag/drop, clipboard, IME, tablet input.
- GPU diagnostics panel (driver, backend, fallback mode).

## 9. FFmpeg integration in UX
- В UI не выставлять raw codec complexity по умолчанию.
- Профили экспорта: `Web H.264`, `Editing ProRes`, `Image Sequence EXR/PNG`, `Alpha-safe`.
- Расширенные параметры кодека доступны в `Advanced` и всегда маппятся на FFmpeg.

## 10. Definition of Done for UI v0.1
- Запускается полноценный UI shell с основными панелями.
- Можно импортировать медиа, создать композицию, анимировать transform, просмотреть и экспортировать.
- Сохраняются workspace layouts и hotkey profile.
- Базовые сценарии стабильны на Linux, Windows, macOS.

