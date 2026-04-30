# Feature Specification: Status Quo Baseline

**Feature Branch**: `001-status-quo-baseline`  
**Created**: 2026-04-30  
**Status**: Baseline  
**Input**: User description: "Create a spec from the existing codebase as if the status quo is the complete spec."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - View Top Market Tokens (Priority: P1)

A visitor opens the app and sees a current, scannable list of the top cryptocurrency market assets with price, 24-hour movement, market cap, volume, and seven-day movement.

**Why this priority**: This is the primary product value of the app.

**Independent Test**: Open the Home page with available market data and verify the token list renders as the main content.

**Acceptance Scenarios**:

1. **Given** market data is available, **When** a visitor opens the Home page, **Then** the app shows 20 ranked token rows.
2. **Given** a token has local artwork, **When** its row renders, **Then** the app shows the local token icon instead of relying on the remote image URL.
3. **Given** a token row is visible, **When** the visitor selects the token name or symbol, **Then** the app opens that token's CoinGecko page in a new browser context.
4. **Given** seven-day sparkline data is available, **When** a token row renders, **Then** the row shows a compact line chart and accessible movement label.

---

### User Story 2 - Understand Loading, Cache, And Refresh State (Priority: P1)

A visitor receives clear feedback while market data loads, when cached data is used, when online data is used, and when loading fails.

**Why this priority**: The app depends on online data and local cache behavior, so users need visible feedback rather than silent waiting.

**Independent Test**: Load the app from a fresh session, from an already cached session, and during an unavailable market-data state.

**Acceptance Scenarios**:

1. **Given** token data is still loading, **When** the Home page is active, **Then** the app displays loading feedback through the toast region.
2. **Given** cached token data is used, **When** the Home page renders, **Then** the app communicates the data source as `Database`.
3. **Given** freshly fetched online token data is used, **When** the Home page renders, **Then** the app communicates the data source as `Online`.
4. **Given** token data cannot be loaded, **When** the request fails, **Then** the app shows an error state in the page and an error toast.
5. **Given** the visitor triggers database repopulation, **When** the refresh request starts, **Then** the app clears stale loading state and shows fresh progress feedback.

---

### User Story 3 - Personalize App Chrome Locally (Priority: P2)

A visitor can switch theme and language preferences, and those preferences remain local across app sessions.

**Why this priority**: Theme and language affect the whole app experience but are secondary to the token list itself.

**Independent Test**: Change theme and language, reload the app, and verify the selected preferences remain active.

**Acceptance Scenarios**:

1. **Given** a visitor selects the theme control, **When** the current theme changes, **Then** the app updates the visual theme and remembers the selection locally.
2. **Given** a visitor opens the language menu, **When** they select English, Spanish, Portuguese, or French, **Then** static app chrome changes to that language and remembers the selection locally.
3. **Given** market token data is rendered, **When** the language changes, **Then** token names, symbols, prices, and market values remain the original market data.

---

### User Story 4 - Navigate Supporting Pages (Priority: P3)

A visitor can navigate to supporting Video, About, and Contact pages without losing the shared app shell.

**Why this priority**: Supporting content improves context and contact paths but is not the primary market-data task.

**Independent Test**: Use the top navigation to visit every route and verify each page renders with the shared navigation and footer.

**Acceptance Scenarios**:

1. **Given** the app is open, **When** the visitor selects Home, Video, About, or Contact, **Then** the matching route content is shown inside the shared shell.
2. **Given** the Video route is active, **When** the page renders, **Then** it displays the embedded video.
3. **Given** the About route is active, **When** the page renders, **Then** it displays profile, Rust, and Dioxus content with local imagery.
4. **Given** the Contact route is active, **When** the page renders, **Then** it displays contact-form and LinkedIn actions as external links.

### Edge Cases

- If no usable local cache exists, the app must attempt online market data.
- If cached data exists but lacks complete sparkline data, the app must prefer a fresh online load.
- If browser database startup is delayed or unavailable, the app must still render from browser snapshot or online data when possible.
- If online data fails and no usable cache exists, the app must present a readable error state.
- If optional token values are missing, the app must show a not-available value rather than an empty or broken cell.
- If sparkline values are missing or invalid, the app must show an unavailable sparkline state.
- If a viewport is narrow, the top navigation must remain usable without wrapping into an incoherent layout.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST provide Home, Video, About, and Contact routes in that order in the top navigation.
- **FR-002**: The system MUST show the shared top navigation, toast region, page content area, and footer across all routes.
- **FR-003**: The system MUST load and display the top 20 market assets ordered by market rank when market data is available.
- **FR-004**: Each token row MUST display rank, icon, name, symbol, current price, 24-hour percentage change, market cap, total volume, and seven-day movement.
- **FR-005**: Token names and symbols MUST link to the corresponding external market-data detail page.
- **FR-006**: The system MUST use bundled token icons for known supported assets and fall back to the market-data image when no local icon exists.
- **FR-007**: The system MUST display positive and negative movement with distinct visual treatments.
- **FR-008**: The system MUST display compact seven-day sparkline charts when enough valid sparkline data exists.
- **FR-009**: The system MUST show an explicit unavailable value for missing optional market fields.
- **FR-010**: The system MUST communicate market-data source status to users as `Database` or `Online`.
- **FR-011**: The system MUST show loading, success, and error feedback through toast-style status messages.
- **FR-012**: The system MUST allow users to manually request a token database repopulation or online refresh from the app chrome.
- **FR-013**: The system MUST persist usable token data locally so later app loads can render without waiting for a fresh online request when possible.
- **FR-014**: The system MUST prefer cached data only when it contains usable sparkline data.
- **FR-015**: The system MUST persist browser token snapshots separately from the browser database so the browser UI can render before database startup completes.
- **FR-016**: The system MUST persist theme preference locally and default to dark theme when no preference exists.
- **FR-017**: The system MUST persist language preference locally and default to English when no preference exists.
- **FR-018**: The system MUST localize static app chrome for English, Spanish, Portuguese, and French.
- **FR-019**: The system MUST keep externally sourced token names, symbols, prices, and market values unchanged by localization.
- **FR-020**: The Video page MUST display the configured embedded video inside the app shell.
- **FR-021**: The About page MUST display profile, Rust, and Dioxus sections using local assets.
- **FR-022**: The Contact page MUST provide external contact-form and LinkedIn actions.
- **FR-023**: External links MUST open outside the current app context and avoid passing opener access.
- **FR-024**: The web and desktop app surfaces MUST preserve equivalent user-facing behavior for shared routes, preferences, market data, and supporting content.
- **FR-025**: The system MUST expose a GitHub repository link from the app chrome.

### Key Entities *(include if feature involves data)*

- **Token**: A market asset with identity, symbol, display name, image, current price, optional rank, optional market cap, optional total volume, optional 24-hour movement, and optional seven-day sparkline values.
- **Token Load Result**: The current token list plus source information and timestamps for online load and local database load.
- **Token Source**: The origin of displayed market data, represented to users as either `Database` or `Online`.
- **Theme Preference**: A local user selection between light and dark visual themes.
- **Language Preference**: A local user selection among English, Spanish, Portuguese, and French.
- **Toast**: A transient status message with tone and identity used for loading, success, and error feedback.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A visitor can open the Home page and identify at least 20 ranked token entries when market data is available.
- **SC-002**: A returning visitor with usable cached data can see token rows without requiring a successful new online request.
- **SC-003**: A manual refresh visibly enters a loading state and then shows either updated data-source feedback or an error message.
- **SC-004**: A visitor can switch theme and language, reload the app, and observe the same preferences still active.
- **SC-005**: A visitor can navigate to Home, Video, About, and Contact from the top navigation with no full app shell loss.
- **SC-006**: Every supported language has non-empty text for all visible static app chrome.
- **SC-007**: Missing optional market values and missing sparkline data are represented by explicit unavailable states rather than broken or blank UI.

## Assumptions

- The app targets users who want a compact market overview rather than account-based portfolio management.
- User authentication, watchlists, trading, alerts, and account storage are outside the current baseline.
- The canonical market-data source for the baseline is CoinGecko-compatible top-market data.
- Local persistence is user-device local and does not sync across devices.
- The browser and desktop apps should remain behaviorally aligned for shared user-facing features.
- The baseline spec documents the current complete product behavior; future specs should describe deltas from this baseline.
