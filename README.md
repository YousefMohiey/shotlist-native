# Shotlist — native shells

Thin native wrappers around a self-hosted Shotlist server (Windows, macOS, iOS, Android)
built with **Tauri v2**. The app itself lives on your server; these shells give it a real
window, icon, installer and home-screen presence without duplicating any data.

```
ui/                     tiny local page: shows "Connecting…", then opens ui/config.js → SHOTLIST_URL
src-tauri/              Rust shell (window, icons, bundle config)
.github/workflows/      free macOS runners build the .dmg and the unsigned .ipa
```

## Point it at your server

Edit **`ui/config.js`** — one line:

```js
window.SHOTLIST_URL = "https://shotlist-yourname.duckdns.org";
```

Use **https** for mobile: iOS (App Transport Security) and Android (cleartext blocked) both
refuse plain `http://` URLs.

## Windows

```bash
npm install
npm run build:win        # → src-tauri/target/release/bundle/nsis/Shotlist_1.0.0_x64-setup.exe
```

Requires Rust (MSVC toolchain), VS Build Tools and the WebView2 runtime (preinstalled on Win 11).
The installer is unsigned, so SmartScreen shows "unknown publisher" until you sign it.

## macOS / iOS without a Mac

Push this repo publicly and run **Actions → native-builds → Run workflow**:

- `Shotlist-macos-unsigned` — universal `.dmg` built on a free `macos-14` runner
- `Shotlist-ios-unsigned` — `.ipa` archived with `CODE_SIGNING_ALLOWED=NO`

Then:

- **macOS**: drag the `.app` to Applications; first launch → right-click → **Open**, or
  `xattr -dr com.apple.quarantine /Applications/Shotlist.app` (unsigned apps are quarantined).
- **iOS**: sideload the `.ipa` with [Sideloadly](https://sideloadly.io) on Windows using your
  normal Apple ID. Free signing lasts **7 days** and allows 3 apps; a paid Apple Developer
  account ($99/yr) gives 1 year and TestFlight.

Free minutes: macOS runners are free for public repositories; on private repos they bill at
~$0.06/min (your 2,000 free minutes ≈ 200 macOS minutes).

## Android

```bash
export ANDROID_HOME=$HOME/android-sdk NDK_HOME=$HOME/android-sdk/ndk/<version> JAVA_HOME=<jdk>
npm run android:init
npm run android:build    # → src-tauri/gen/android/app/build/outputs/apk/universal/release/*.apk
```

Sideload the APK, or publish to Play (one-off $25).

## Notes

- The shell loads your server, so **sign-in and data are shared** with the web app — nothing is
  stored locally except the WebView cache.
- Apple may reject a pure webview app under App Store guideline 4.2; TestFlight/sideload avoids review.
- No secrets live in this repo. The server's `.env` (Gemini key) stays on the server.
