---
name: ZarishNote
colors:
  surface: '#0b1326'
  surface-dim: '#0b1326'
  surface-bright: '#31394d'
  surface-container-lowest: '#060e20'
  surface-container-low: '#131b2e'
  surface-container: '#171f33'
  surface-container-high: '#222a3d'
  surface-container-highest: '#2d3449'
  on-surface: '#dae2fd'
  on-surface-variant: '#cbc3d7'
  inverse-surface: '#dae2fd'
  inverse-on-surface: '#283044'
  outline: '#958ea0'
  outline-variant: '#494454'
  surface-tint: '#d0bcff'
  primary: '#d0bcff'
  on-primary: '#3c0091'
  primary-container: '#a078ff'
  on-primary-container: '#340080'
  inverse-primary: '#6d3bd7'
  secondary: '#4cd7f6'
  on-secondary: '#003640'
  secondary-container: '#03b5d3'
  on-secondary-container: '#00424e'
  tertiary: '#4edea3'
  on-tertiary: '#003824'
  tertiary-container: '#00a572'
  on-tertiary-container: '#00311f'
  error: '#ffb4ab'
  on-error: '#690005'
  error-container: '#93000a'
  on-error-container: '#ffdad6'
  primary-fixed: '#e9ddff'
  primary-fixed-dim: '#d0bcff'
  on-primary-fixed: '#23005c'
  on-primary-fixed-variant: '#5516be'
  secondary-fixed: '#acedff'
  secondary-fixed-dim: '#4cd7f6'
  on-secondary-fixed: '#001f26'
  on-secondary-fixed-variant: '#004e5c'
  tertiary-fixed: '#6ffbbe'
  tertiary-fixed-dim: '#4edea3'
  on-tertiary-fixed: '#002113'
  on-tertiary-fixed-variant: '#005236'
  background: '#0b1326'
  on-background: '#dae2fd'
  surface-variant: '#2d3449'
typography:
  headline-lg:
    fontFamily: Inter
    fontSize: 32px
    fontWeight: '700'
    lineHeight: '1.2'
    letterSpacing: -0.02em
  headline-md:
    fontFamily: Inter
    fontSize: 24px
    fontWeight: '600'
    lineHeight: '1.3'
    letterSpacing: -0.01em
  body-lg:
    fontFamily: Inter
    fontSize: 18px
    fontWeight: '400'
    lineHeight: '1.6'
  body-md:
    fontFamily: Inter
    fontSize: 16px
    fontWeight: '400'
    lineHeight: '1.6'
  code-md:
    fontFamily: JetBrains Mono
    fontSize: 14px
    fontWeight: '400'
    lineHeight: '1.5'
  label-sm:
    fontFamily: JetBrains Mono
    fontSize: 12px
    fontWeight: '500'
    lineHeight: '1.0'
    letterSpacing: 0.05em
  headline-lg-mobile:
    fontFamily: Inter
    fontSize: 26px
    fontWeight: '700'
    lineHeight: '1.2'
rounded:
  sm: 0.125rem
  DEFAULT: 0.25rem
  md: 0.375rem
  lg: 0.5rem
  xl: 0.75rem
  full: 9999px
spacing:
  unit: 4px
  gutter-md: 24px
  margin-page: 32px
  container-max: 800px
---

## Brand & Style

The design system is built on the philosophy of **Cyber-minimalism**. It prioritizes a high-performance, distraction-free environment for deep work, blending the austerity of a professional terminal with the sophistication of modern high-end hardware interfaces.

The target audience consists of power users, developers, and researchers who value privacy and speed. The UI should feel like a "sandboxed" digital vault—secure, local-first, and highly responsive. 

**Visual Principles:**
- **Minimalism:** Aggressive reduction of UI chrome. Functionality is revealed through intent.
- **Glassmorphism:** Semi-transparent panels with high-blur backdrops to maintain a sense of depth and spatial awareness without visual clutter.
- **Precision:** Mathematical alignment and consistent technical details that evoke a sense of high-fidelity engineering.
- **Subtle Glow:** Accents use soft, outer-glow properties to signify AI activity or active states, mimicking the light emitted from high-end electronics.

## Colors

The palette is strictly dark-mode, designed to reduce eye strain during long writing sessions and emphasize the "obsidian vault" metaphor.

- **Foundational Neutrals:** Use Obsidian (`#020617`) for the primary canvas and Deep Charcoal (`#0F172A`) for elevated panels or sidebars.
- **Accents:** 
    - **Electric Violet (#8B5CF6):** Used for primary actions, selection states, and core branding.
    - **Neon Cyan (#06B6D4):** Dedicated specifically to AI-powered features, highlighting, and generative states.
- **Semantic Colors:** Success states utilize a sharp Emerald (`#10B981`), while errors use a high-contrast Ruby (`#E11D48`).
- **Surface Overlays:** Use white with extremely low alpha (2% to 6%) for glass surfaces to create subtle light-traps on dark backgrounds.

## Typography

This design system utilizes a dual-font strategy to balance readability with a technical aesthetic.

- **Primary Typeface (Inter):** Used for all prose, headings, and interface elements to ensure maximum legibility and a modern feel.
- **Technical Typeface (JetBrains Mono):** Reserved for code blocks, Markdown syntax hints, metadata labels, and status bar information. This reinforces the "technical" nature of the editor.

**Scaling & Hierarchy:**
- Headlines use tighter letter spacing and heavier weights to feel "locked in."
- Body text uses a generous line height (1.6) to provide a comfortable "breathing room" for long-form writing.
- Labels are always rendered in JetBrains Mono, frequently in uppercase with slight tracking to distinguish them from content.

## Layout & Spacing

The layout follows a **Fixed-Fluid hybrid model**. The writing canvas is restricted to a maximum width of 800px to maintain an ideal line length for readability, while the UI chrome (sidebars, status bars) remains fluid.

- **Grid:** A 4px baseline grid governs all vertical and horizontal rhythm.
- **The Sidebar:** Collapsible by default. When active, it uses a glassmorphic background with a 240px fixed width.
- **Margins:** Desktop views utilize a 32px safe-area margin. Mobile views drop to 16px.
- **Guttering:** Content sections are separated by 24px gutters to prevent visual crowding.
- **AI Sidebar:** A secondary, right-aligned panel (320px) may appear for AI chat or transformations, pushing the main content to the left to maintain balance.

## Elevation & Depth

Hierarchy is established through **Luminance and Blur** rather than traditional shadows.

- **Level 0 (Base):** Obsidian (`#020617`). The deep background of the application.
- **Level 1 (Sub-surface):** Deep Charcoal (`#0F172A`). Used for gutters or inactive sidebar states.
- **Level 2 (Glass Layer):** `background: rgba(15, 23, 42, 0.6)` with a `backdrop-filter: blur(12px)`. Used for floating toolbars, modals, and the primary sidebar.
- **Level 3 (Interactive):** Elements that are active or being hovered receive a 1px inner border (`rgba(255,255,255,0.1)`) to catch the light.
- **Glow Effects:** AI-powered components or the active cursor in "Focus Mode" use a subtle `box-shadow: 0 0 15px rgba(6, 182, 212, 0.3)` to signify the "powered" state.

## Shapes

The shape language is "Soft-Technical." Elements use a subtle radius (4px) to avoid the aggression of sharp 90-degree corners, while remaining distinct and disciplined.

- **Standard Elements (Buttons, Inputs):** 4px (`rounded-md`).
- **Containers (Cards, Panels):** 8px (`rounded-lg`).
- **Interactive Indicators:** 2px (`rounded-sm`). Small status dots or selection indicators.
- **Avoid Pills:** Do not use pill-shaped buttons; they are too organic for the cyber-minimalist aesthetic. Stick to soft-cornered rectangles.

## Components

### Buttons & Inputs
- **Primary Action:** Solid Electric Violet with white text. No shadow, but a 1px light-violet top border for "rim lighting."
- **AI Action:** Neon Cyan border (1px) with a translucent background. On hover, apply a soft Cyan outer glow.
- **Ghost Input:** Input fields have no background or border by default. They rely on the `code-md` typography and a 1px bottom border that activates on focus.

### Cards & Surfaces
- **Glass Card:** Used for file previews or AI suggestions. Features a 1px border (`rgba(255,255,255,0.05)`) and a 12px backdrop blur.

### AI Interface
- **AI Sparkline:** A thin, pulsating 2px line of Neon Cyan that appears at the top of the editor when the AI is processing.
- **Contextual Menu:** Appears on text selection. Dark, blurred glass background with minimalist icons.

### Editor Specifics
- **Markdown Syntax:** Should be rendered in 50% opacity neutral text to keep the focus on the content.
- **Active Line:** Highlighted with a subtle Deep Charcoal background and a 2px Electric Violet vertical bar in the left gutter.