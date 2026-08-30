# 🚀 PawsMC Quickstart Guide & Instrukcja dla Użytkownika

Ten przewodnik wyjaśnia krok po kroku, jak przygotować środowisko deweloperskie i jak użyć przygotowanych plików z promptami do bezbłędnego zrealizowania forka **PawsMC** przez **Claude Opus**.

---

## 📁 Przygotowany Zestaw Plików w Projekcie

W tym folderze (`d:\Dev\Minecraft Launcher\`) znajdują się wszystkie niezbędne dokumenty i prompty:

| Plik | Przeznaczenie |
| :--- | :--- |
| **`PAWSMC_MASTER_PROMPT.md`** | **Główny prompt dla Claude Opus.** Wklej jego zawartość do Claude'a na start. Zawiera wszystkie reguły, architekturę i wymagania. |
| **`PAWSMC_ARCHITECTURE_SPEC.md`** | Specyfikacja techniczna repozytorium `modrinth/theseus`, wykaz funkcji IPC Tauri i tabela rebrandingu. |
| **`PAWSMC_UI_UX_DESIGN_SYSTEM.md`** | Kompletny Design System nowego interfejsu (kolory, glassmorphism, tokeny CSS, komponenty). |
| **`PAWSMC_STEP_BY_STEP_EXECUTION_PLAN.md`** | Plan wykonawczy z podziałem na 8 faz do weryfikacji postępów. |
| **`PAWSMC_QUICKSTART_GUIDE.md`** | Ten plik – instrukcja konfiguracji środowiska i komend. |

---

## 🛠️ 1. Wymagania Wstępne (Prerequisites)

Upewnij się, że na Twoim komputerze z systemem Windows zainstalowane są:

1. **Rust & Cargo**:
   - Sprawdź: `rustc --version` i `cargo --version`
   - Jeśli brak: Pobierz i zainstaluj z [rustup.rs](https://rustup.rs/).
2. **Node.js & pnpm**:
   - Sprawdź: `node -v` i `pnpm -v`
   - Zalecana wersja Node: `>= 20.x`.
   - Jeśli brak pnpm: `npm install -g pnpm`
3. **C++ Build Tools (Windows)**:
   - Microsoft C++ Build Tools (zainstalowane automatycznie przez Visual Studio Installer lub Rustup).
4. **Tauri CLI**:
   - `pnpm add -D @tauri-apps/cli` lub `cargo install tauri-cli`

---

## 🤖 2. Jak przekazać zadanie do Claude Opus

Możesz przekazać zadanie Claude Opus na dwa sposoby:

### Sposób A: Praca w sesji / czacie z Claude Opus
1. Otwórz czat z Claude 3.5 Sonnet / Claude Opus.
2. Skopiuj całą zawartość pliku **`PAWSMC_MASTER_PROMPT.md`** i wklej jako pierwszą wiadomość.
3. Jeśli Claude poprosi o dodatkowe szczegóły dotyczące UI lub architektury, dołącz odpowiednio `PAWSMC_UI_UX_DESIGN_SYSTEM.md` lub `PAWSMC_ARCHITECTURE_SPEC.md`.

### Sposób B: Praca z agentem CLI / IDE (np. Claude Code lub Antigravity z Claude Opus)
1. Uruchom agenta w folderze roboczym `d:\Dev\Minecraft Launcher`.
2. Podaj mu prompt:
   > *"Przeczytaj pliki `PAWSMC_MASTER_PROMPT.md`, `PAWSMC_ARCHITECTURE_SPEC.md`, `PAWSMC_UI_UX_DESIGN_SYSTEM.md` oraz `PAWSMC_STEP_BY_STEP_EXECUTION_PLAN.md`. Sklonuj repozytorium `modrinth/theseus`, przeprowadź kompletny rebranding na PawsMC i zaimplementuj fazy od 1 do 8 zgodnie z planem."*

---

## ⚡ 3. Kluczowe Komendy Deweloperskie

Gdy projekt zostanie zainicjowany:

### Klonowanie bazy Modrintha:
```powershell
git clone https://github.com/modrinth/theseus.git .
```

### Instalacja zależności frontendu:
```powershell
pnpm install
```

### Uruchomienie trybu deweloperskiego (Live Reload):
```powershell
pnpm tauri dev
```
*(Ta komenda automatycznie skompiluje backend Rusta i uruchomi okno Tauri z Vite/Vue live reload).*

### Weryfikacja poprawności TypeScript i kompilacji:
```powershell
pnpm build
cargo check --manifest-path src-tauri/Cargo.toml
```

### Zbudowanie gotowego instalatora (.msi / .exe):
```powershell
pnpm tauri build
```
Gotowe pliki instalatora pojawią się w folderze `src-tauri/target/release/bundle/msi/` lub `nsis/`.

---

## 💡 4. Wskazówki & Dobre Praktyki

1. **Nie modyfikuj sygnatur IPC bez potrzeby**: Backend Rusta w `theseus` jest wysoce zoptymalizowany. Najlepiej skupić się na wymianie warstwy prezentacji w Vue 3 i stylach CSS, zachowując istniejące Pinia stores i wywołania `invoke()`.
2. **Konta Microsoft**: Podczas logowania Microsoftu aplikacja stawia lokalny serwer HTTP na wolnym porcie i odbiera kod OAuth z przeglądarki. Ten mechanizm działa w 100% out-of-the-box i nie wymaga własnego serwera autoryzacji.
3. **Pobieranie Javy**: Modrinth posiada wbudowany moduł automatycznego pobierania Javy (Java 8, 17, 21 dla odpowiednich wersji Minecrafta) z Adoptium API – PawsMC w pełni dziedziczy tę funkcjonalność.
