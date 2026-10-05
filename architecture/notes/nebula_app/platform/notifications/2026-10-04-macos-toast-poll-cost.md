# macOS actionable toasts share one auto-dismiss poll

## Status

Proposed for review with implementation.

## Context

Actionable toasts on macOS (`platform::notifications::toast_actionable`) call
notify-rust's `wait_for_response` on a `pebrel-toast` thread. On the
NSUserNotification backend (notify-rust 4.18.0 -> mac-notification-sys 0.6.15)
every waiting send starts a repeating 0.5 s timer on the main run loop that calls
`-[NSUserNotificationCenter deliveredNotifications]`, the only signal that a
notification left Notification Center without a click. The call is a synchronous
XPC round trip, and its reply, the whole delivered list, is decoded on every call.
A toast nobody answers keeps its timer until it is clicked, dismissed or cleared,
so ignored toasts accumulate.

## Evidence

- `sample` of the running 2.1.0 build on 2026-10-04 (5 s at 5 ms): 313 distinct
  `com.apple.NSUserNotificationCenter.SyncQueue` threads (about 62 polls/s), 32
  parked `pebrel-toast` threads, and the main thread inside
  `-[_NSConcreteUserNotificationCenter deliveredNotifications]` in 666 of 786
  samples. Pebrel used about 45% of a core, `usernoted` about 30% and WindowServer
  17-23% on a machine that was about 90% idle overall.
- 32 pending x 2 polls/s = 64 polls/s, matching the observed 62. At about 13 ms
  per call that is the observed 85% of the main thread. Cost grows with pending
  toasts times list length, so the main thread saturates near 40 pending toasts.
- Upstream source: `objc/notify.m`, non-main-thread branch of `sendNotification`.
- Windows already bounds retention
  (`2026-09-20-actionable-notification-lifetime.md`); macOS had no bound.

## Decision

Vendor `mac-notification-sys` 0.6.15 at `third_party/mac-notification-sys-0.6.15`
through `[patch.crates-io]`, the mechanism already used for `winit`. Only
`objc/notify.m` changes:

- Waiting notifications register with one poller on a private serial queue
  instead of each owning a main-run-loop timer.
- A tick fetches `deliveredNotifications` once for all of them, off the main
  thread.
- The next tick is scheduled no sooner than 20x the time that fetch took, clamped
  to 0.5-10 s, so polling stays near 5% of one core however long the list gets.
- Resolution (`resolveAutoDismiss`) still runs from a main run loop callout via
  `-[NSRunLoop performBlock:]`, so a delegate callback that is already queued
  still wins over an auto-dismiss.
- Per-notification rules are unchanged: delivery must be confirmed before absence
  counts, and unconfirmed delivery resolves after 2 s.

The Rust bridge, the crate's public API and `platform/notifications.rs` are
unchanged.

## Rejected alternatives

- Cap pending waiting toasts: overflow toasts lose click-to-focus, and list length
  still drives the cost of each poll.
- UNUserNotificationCenter (notify-rust feature `preview-macos-un`): new
  dependency, authorization prompt and signing requirements on ad-hoc-signed local
  builds, and not verifiable by compilation.
- Clear delivered notifications when the window gains focus: system toasts are
  also sent while the window is active but the source pane is in another tab, and
  it would remove unseen ones.
- Suppress native toasts while the window is active: changes notification policy;
  a separate decision.
- Keep per-notification timers and slow them down: cost still multiplies by the
  pending count.

## Consequences

- One parked thread per unanswered toast remains: cheap, but unbounded in
  principle.
- Auto-dismiss detection can lag up to 10 s with very long lists. It only decides
  when the waiting thread ends, not what the user sees or whether clicks work.
- A vendored third-party copy has to be carried until upstream ships an equivalent
  fix. The crates.io package contains no license file, so none was vendored; the
  `license = "MIT/Apache-2.0"` metadata in its `Cargo.toml` is preserved.

## Validation

- The Objective-C compiles without warnings under `-Wall -Wextra`;
  `cargo check --locked --offline -p mac-notification-sys` passes, and
  `cargo tree` shows the app resolving the patched crate through notify-rust.
- A throwaway harness compiled against the patched `notify.m`, with a fake
  notification list and fake Rust callbacks (not committed), checked: 40 pending
  notifications cost 4 list fetches in 2.4 s instead of about 190; a notification
  that vanishes resolves exactly once on the main thread; one completed by a real
  callback is not resolved; unconfirmed delivery resolves after the 2 s timeout; a
  0.1 s fetch backs the interval off to about 2 s; the poller goes idle when
  nothing is registered.
- Not yet verified on a device with a packaged build. Acceptance: with at least 32
  unanswered toasts, `sample <pid> 5 5` shows about 10 or fewer `SyncQueue`
  threads per 5 s (was 313), main-thread time in `deliveredNotifications` below 5%
  (was 85%), and Pebrel and `usernoted` CPU below about 5% each (were about 45% and
  30%); clicking a toast, pressing an action button and dismissing a toast still
  work.

## Supersedes

None

## Revisit when

Upstream mac-notification-sys ships a shared or backed-off poll, or Pebrel moves to
UNUserNotificationCenter.
