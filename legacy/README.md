# oci-arm-catcher — classic scripts (legacy)

These are the original bash and PowerShell versions of oci-arm-catcher. They
still work, but the **[Rust binary](../README.md)** replaces them: one file, no
OCI CLI, no Python, same `.env`. New features land there; the scripts only get
fixes.

> Until the move to `legacy/`, these files lived in the repo root. That layout is
> preserved at the [`scripts-final`](https://github.com/deeplabua/oci-arm-catcher/tree/scripts-final)
> tag, if you're following an older guide.

## What's here

| File | What it does |
|---|---|
| `oci-arm-catcher.sh` | The catcher for macOS / Linux (bash + OCI CLI + Python 3) |
| `oci-arm-catcher.ps1` | Native Windows PowerShell port (5.1+) |
| `scripts/get-config.sh`, `.ps1` | Read-only helper that prints the OCIDs you need |
| `tests/` | bats and Pester tests with a mocked `oci` CLI |

## Prerequisites

1. **An Oracle Cloud account** and a **VCN with a public subnet** (the console's
   "Create VCN with Internet Connectivity" wizard).
2. **The OCI CLI**, installed and configured:

   ```bash
   brew install oci-cli          # macOS
   pip install oci-cli           # Linux / Windows / anything with pip
   oci setup config              # one-time, creates ~/.oci/config
   oci iam region list           # check it works
   ```

3. **Python 3** for the bash version (already on macOS and most Linux distros).

## Quick start

From the repo root:

```bash
cp .env.example .env               # then edit it
./legacy/scripts/get-config.sh     # read-only: prints the OCIDs for .env
./legacy/oci-arm-catcher.sh
```

Windows (PowerShell):

```powershell
Copy-Item .env.example .env
.\legacy\scripts\get-config.ps1
.\legacy\oci-arm-catcher.ps1
```

The scripts read `legacy/.env` if it exists, otherwise the `.env` in the repo
root, or a path you pass (`./legacy/oci-arm-catcher.sh my.env`,
`-ConfigFile my.env`).

## Script-only settings

Everything in [`.env.example`](../.env.example) applies. The scripts also read:

| Variable | Default | Notes |
|---|---|---|
| `ERROR_LOG` | `legacy/oci-arm-catcher-errors.log` | Raw OCI CLI output of every failed attempt is appended here |
| `MAX_CONSECUTIVE_UNKNOWN` | `0` (off) | Stop after this many unrecognised errors in a row |

## How errors are handled

The scripts match the OCI CLI's error text: a deny-list of fatal errors (auth,
bad OCID, `LimitExceeded`, CLI usage errors) stops the run; everything else is
retried, so an unfamiliar transient error can't silently end a long hunt.

## Running in the background

```bash
nohup ./legacy/oci-arm-catcher.sh > catcher.log 2>&1 &
tail -f catcher.log
```

```powershell
Start-Job -ScriptBlock { & "C:\path\to\legacy\oci-arm-catcher.ps1" }
```

The bash script prints a once-a-minute heartbeat when writing to a log file,
and a live countdown in a terminal.

On Windows you can also run the bash version under
[WSL2](https://learn.microsoft.com/windows/wsl/install) or Git Bash, with
`python3` available there.

## Troubleshooting

**`'oci' is not installed or not on PATH`**
Install the OCI CLI (above), then check with `oci iam region list`. With
Homebrew you may need to restart your terminal.

**It looks frozen — the spinner keeps spinning**
Old copies could hang inside one attempt: the OCI CLI's Python SDK retries on
its own and can loop on `429 Too Many Requests` for minutes. The scripts pass
`--no-retry` and do their own retrying (thanks
[@zearp](https://github.com/zearp), [#1](https://github.com/deeplabua/oci-arm-catcher/issues/1)).
Update with `git pull`.

**`get-config.ps1` fails with "Unexpected token" on Windows**
Fixed in [#6](https://github.com/deeplabua/oci-arm-catcher/pull/6): the
PowerShell files are now pure ASCII, which Windows PowerShell 5.1 needs.
Update with `git pull`.

**Notifications don't show up**
They're best-effort: `osascript` on macOS, `notify-send` (libnotify) on Linux,
[BurntToast](https://github.com/Windos/BurntToast) or a tray balloon on Windows.
The console output is always there.

## Tests

```bash
brew install bats-core shellcheck     # or: apt-get install bats shellcheck

bats legacy/tests/
shellcheck legacy/oci-arm-catcher.sh legacy/scripts/get-config.sh
pwsh -c "Invoke-Pester -Path legacy/tests/oci-arm-catcher.Tests.ps1"
```

The bash script can be sourced in library mode
(`OCI_ARM_CATCHER_LIB=1 source legacy/oci-arm-catcher.sh`), which defines its
helpers without running the loop. All of this runs in CI.
