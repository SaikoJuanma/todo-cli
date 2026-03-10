#!/usr/bin/env bash
set -euo pipefail

echo "[setup-rust] Starting..."

# Inputs opcionales desde action.yml / workflow
TOOLCHAIN="${TOOLCHAIN:-stable}"
TARGETS="${TARGETS:-}"

# Si tu imagen define estos, los respetamos
export RUSTUP_HOME="${RUSTUP_HOME:-/opt/rustup}"
export CARGO_HOME="${CARGO_HOME:-/opt/cargo}"
export PATH="${CARGO_HOME}/bin:${PATH}"

# ------------------------------------------------------------
# 1) Instalar deps de sistema SOLO si:
#    - somos root
#    - y apt-get existe
#    - y faltan paquetes clave
#    (en tu imagen ya estarán, así que esto casi siempre será NO-OP)
# ------------------------------------------------------------
if command -v apt-get >/dev/null 2>&1 && [ "$(id -u)" -eq 0 ]; then
  echo "[setup-rust] Checking system deps..."
  # Ajusta esta lista si quieres
  DEPS=(ca-certificates curl git build-essential pkg-config libssl-dev)
  # Intento liviano: no fallar si ya están
  apt-get update -y
  apt-get install -y --no-install-recommends "${DEPS[@]}" || true
fi

# ------------------------------------------------------------
# 2) Asegurar Rust
#    - Si cargo ya existe, no reinstalamos nada.
#    - Si no existe, instalamos rustup de forma estándar.
# ------------------------------------------------------------
if command -v cargo >/dev/null 2>&1; then
  echo "[setup-rust] cargo already available: $(cargo --version)"
else
  echo "[setup-rust] Installing rustup (minimal)..."
  # Si el sistema no tiene curl por algún reason, esto fallará claramente
  curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal

  # En instalaciones estándar, el env queda en $HOME/.cargo/env
  # pero NO lo asumimos rígidamente.
fi

# ------------------------------------------------------------
# 3) Source seguro del env (si existe)
# ------------------------------------------------------------
# Primero intenta con CARGO_HOME
if [ -f "${CARGO_HOME}/env" ]; then
  # rustup puede crear esto en algunos setups
  # shellcheck disable=SC1090
  source "${CARGO_HOME}/env"
elif [ -f "${HOME}/.cargo/env" ]; then
  # fallback clásico
  # shellcheck disable=SC1090
  source "${HOME}/.cargo/env"
else
  # No hay env: igual está bien si PATH ya trae cargo
  true
fi

# Reforzamos PATH por si acaso
export PATH="${CARGO_HOME}/bin:${PATH}"

# ------------------------------------------------------------
# 4) Toolchain + componentes
# ------------------------------------------------------------
echo "[setup-rust] Ensuring toolchain: ${TOOLCHAIN}"
rustup toolchain install "${TOOLCHAIN}" || true
rustup default "${TOOLCHAIN}" || true

echo "[setup-rust] Ensuring components..."
rustup component add rustfmt clippy || true

# ------------------------------------------------------------
# 5) Targets opcionales
#    Usa variable TARGETS como CSV: "arm-unknown-linux-gnueabihf,armv7-unknown-linux-gnueabihf"
# ------------------------------------------------------------
if [ -n "${TARGETS}" ]; then
  IFS=',' read -ra TLIST <<< "${TARGETS}"
  for t in "${TLIST[@]}"; do
    t="$(echo "$t" | xargs)"
    [ -n "$t" ] || continue
    echo "[setup-rust] Adding target: $t"
    rustup target add "$t" || true
  done
fi

echo "[setup-rust] Done."

