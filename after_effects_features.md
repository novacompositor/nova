# Функции и возможности Adobe After Effects (Референс для Nova Compositor)

Настоящий документ описывает функционал классического After Effects, который необходимо воссоздать в Nova Compositor для обеспечения полноценного опыта работы композитора и моушен-дизайнера.

## 1. Интерфейс и Рабочее пространство (UI / Workspace)
- **Настраиваемые макеты (Workspaces):** Изменяемые по размеру док-панели (SplitView) с возможностью перетаскивания (Drag & Drop) вкладок, сохранения и загрузки кастомных пресетов рабочих пространств.
- **Главная панель инструментов (Toolbar):** Selection Tool, Hand, Zoom, Rotation, Camera Tools, Pan Behind (Anchor Point), Shape, Pen, Type, Brush, Clone Stamp, Eraser, Roto Brush, Puppet Pin.
- **Project Panel:** Организация ассетов по папкам, предпросмотр медиа, поиск, интерпретация футажей (частота кадров, пиксельные пропорции, альфа-канал).
- **Timeline Panel:** Управление слоями (стыковка, обрезка, перемещение), режимы (Switches / Modes), родительские связи (Parenting / Whip-pick), переключатели слоев (Shy, Solo, Lock, Video, Audio).
- **Composition / Viewer Panel:** Рендер-канвас, зуммирование (от 1.5% до 800%), выбор разрешения (Full, Half, Third, Quarter), безопасные зоны (Title/Action Safe), сетки, направляющие (Rulers & Guides), настройки просмотра каналов (RGB, Alpha, Z-Depth).
- **Info, Audio, Preview Panels:** Инфо о координатах и цвете пикселя под курсором, микшер звука, элементы управления воспроизведением.
- **Effects & Presets Panel:** Дерево эффектов и пресетов анимации с быстрым поиском по названию.

## 2. Слои (Layers) и Модификаторы
Поддержка различных типов слоев с их уникальными свойствами:
- **Video / Image / Audio:** Медиафайлы.
- **Solid Layers:** Плоские заливки цветом, на которые часто вешаются эффекты.
- **Null Objects:** Невидимые слои для управления привязанными (Parent) объектами.
- **Adjustment Layers:** Слои-коррекции, чьи эффекты применяются ко всем слоям, лежащим ниже.
- **Text Layers:** Векторный текст с поддержкой аниматоров (Text Animators: Range Selector, Position, Scale, Opacity и т.д.).
- **Shape Layers:** Векторные примитивы со своими параметрами, группами, и модификаторами (Trim Paths, Repeater, Pucker/Bloat, Twist, Wiggle Paths).
- **Camera Layers:** Виртуальные камеры (1-node, 2-node) с настройками фокусного расстояния, глубины резкости (Depth of Field), диафрагмы и размытия (Blur Level).
- **Light Layers:** Источники света (Point, Spot, Ambient, Parallel) с настройками интенсивности, цвета, отбрасывания теней, спада (Falloff).
- **Pre-compositions:** Возможность вкладывать одну композицию внутрь другой со сворачиванием трансформаций (Collapse Transformations / Continuously Rasterize).

## 3. Анимация и Ключевые Кадры (Keyframes)
- **Типы ключей:** Linear, Bezier, Continuous Bezier, Auto Bezier, Hold.
- **Graph Editor:** Редактор графиков (Value Graph, Speed Graph) для точной настройки кривых скорости и значений. Роуминг ключей (Rove across time).
- **Motion Blur:** Вычислительное размытие в движении на основе угла и фазы затвора (Shutter Angle / Phase).
- **Expressions:** Написание скриптов (JavaScript-подобных) для параметров. Связи через `wiggle`, `time`, `linear`, обращение к другим свойствам. Поддержка Expression Controls (Slider, Color, Checkbox).
- **Time Stretching & Remapping:** Растягивание времени, заморозка кадров (Freeze Frame), управление скоростью клипа по ключам (Time Remap).

## 4. Маски, Каналы и Смешивание (Compositing)
- **Masks:** Векторные маски (Pen Tool / Shape Tool) на любом слое. Операции масок: Add, Subtract, Intersect, Lighten, Darken, Difference. Настройки Feather, Opacity, Expansion. Трекинг масок.
- **Track Mattes:** Использование яркости (Luma Matte) или альфа-канала (Alpha Matte) вышележащего или указанного слоя как маски для нижележащего. Поддержка Inverted.
- **Blend Modes:** Экранные режимы (Normal, Multiply, Screen, Overlay, Add, Soft Light, Color Dodge и др. - всего около 38 режимов).
- **Alpha & Luma:** Извлечение каналов, премноженная (Premultiplied) и прямая (Straight) альфа.

## 5. Эффекты (Effects)
Сотни встроенных эффектов. Базовые категории:
- **Color Correction:** Curves, Levels, Lumetri Color, Hue/Saturation, Tint, Color Balance.
- **Blur & Sharpen:** Gaussian Blur, Box Blur, Fast Box Blur, Camera Lens Blur.
- **Distort:** CC Slant, Mesh Warp, Turbulent Displace, Wave Warp, Displacement Map.
- **Generate:** Gradient Ramp, Stroke, Grid, Audio Spectrum, Fractal Noise.
- **Keying:** Keylight (экранное кеирование), Linear Color Key, Luma Key.
- **Matte / Edge:** Refine Soft Matte, Simple Choker.
- **Simulation:** Particle Systems, CC Rainfall, CC Snow.
- **Stylize:** Glow, Mosaic, Posterize, Find Edges.
- **Time:** Posterize Time, Echo, CC Force Motion Blur.

## 6. 3D Пространство
- **2.5D / 3D Слои:** Превращение 2D слоя в 3D (появление оси Z и ориентации X/Y/Z).
- **Взаимодействие со светом:** Свойства слоя `Accepts Lights`, `Accepts Shadows`, `Casts Shadows`, параметры материала (Ambient, Diffuse, Specular, Shininess).
- **Рендереры:** Classic 3D (базовое Z-сортирование, пересечение слоев), Cinema 4D (выдавливание / Extrusion, искривление Geometry).
- **Depth of Field:** Размытие по глубине в зависимости от расстояния от Камеры.

## 7. Рендеринг и Экспорт
- **Render Queue:** Очередь рендера с модулями вывода (Output Modules) и настройками рендера (Render Settings).
- Распознавание форматов: H.264, ProRes, EXR/PNG Sequences.
- Поддержка рендеринга с альфа-каналом.

## 8. Особые возможности и Инструменты
- **Tracker Panel:** 1-point / 2-point трекинг, стабилизация (Warp Stabilizer), 3D Camera Tracker.
- **Puppet Pin Tool:** Деформация растровых/векторных элементов по точкам.
- **Roto Brush Tool:** ИИ/алгоритмическое выделение объектов из фона.

---
*Этот список является целевой функциональной картой для дальнейшего внедрения в архитектуру Rust + QML.*
