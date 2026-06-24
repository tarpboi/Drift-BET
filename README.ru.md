# Торговый бот Drift BET

**🌐 Language / 语言 / Язык:** [English](README.md) · [简体中文](README.zh-CN.md) · [Русский](README.ru.md)

![Status](https://img.shields.io/badge/status-🟢_live-2ea44f?style=flat-square)
[![Engine](https://img.shields.io/badge/engine-shared_core-6e40c9?style=flat-square)](https://github.com/HarrierOnChain/Prediction-Markets-Trading-Bot-Toolkits)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](LICENSE)
[![CI](https://github.com/HarrierOnChain/Drift-BET/actions/workflows/ci.yml/badge.svg)](https://github.com/HarrierOnChain/Drift-BET/actions/workflows/ci.yml)

> Автоматический **торговый бот Drift BET** — Solana. Часть набора [Prediction Market Toolkits](https://github.com/HarrierOnChain/Prediction-Markets-Trading-Bot-Toolkits): одно ядро исполнения, один слой риска, все площадки.

**Drift BET** уже **работает в продакшене** сегодня.

---

## Drift BET в реальной торговле

<div align="center">

<img width="820" alt="Drift BET trading bot TUI" src="https://github.com/user-attachments/assets/b6c51ba1-14c6-4582-858c-e9441516dd1d" />
<img width="820" alt="Drift BET trading bot TUI" src="https://github.com/user-attachments/assets/66d9cb72-e14a-414f-93e5-600fb1d3f49f" />

<sub>Общий TUI в работе на Drift BET — позиции в реальном времени, P&L и состояние предохранителя. <!-- TODO: swap in Drift BET-specific captures --></sub>

</div>

---

## Стратегии на Drift BET

Эти боты работают на Drift BET через один адаптер площадки на общем движке — те же риск-контроли, тот же TUI, полная поддержка dry-run.

| Стратегия |
|----------|
| ⚡ **BTC арбитраж 5м / 15м / 1ч** — скорость на коротких BTC Up/Down (~42 мс от начала до конца) |
| 📊 **Дисбаланс стакана** — сигнал *и есть* стакан заявок, без внешних фидов |
| 💰 **Маркет-мейкинг** — будьте казино, а не игроком (двусторонний GTD, перекос инвентаря) |
| ⚡ **Ончейн-сигнал китов** — на 3–30 с раньше публичного API позиций |

> Нужна стратегия, не указанная здесь для Drift BET? Покрытие адаптеров определяется спросом — [напишите](https://t.me/HarrierOnChain).

---

## Быстрый старт

Клонируйте, добавьте ключи и запустите — TUI даст выбрать стратегию.

```bash
git clone https://github.com/HarrierOnChain/Drift-BET.git
cd Drift-BET
cp config.example.yaml config.yaml   # add your keys
cargo run --release                  # launch the TUI
# headless: cargo run --release -- run copy-trading
```

---

## Один движок, все площадки

Этот репозиторий — точка входа для **Drift BET**. Ядро исполнения, слой риска и все 20+ адаптеров площадок находятся в основном наборе:

### 👉 **[Prediction-Markets-Trading-Bot-Toolkits](https://github.com/HarrierOnChain/Prediction-Markets-Trading-Bot-Toolkits)** — полный набор

| | |
|---|---|
| **Исполнение ордеров** | < 100 мс от начала до конца |
| **Обработка событий** | < 1 мс на событие |
| **Безопасность** | Предохранитель · защита глубины · dry-run · минимум сделки |
| **Площадки** | 7 активны · 20+ площадок |

Добавить площадку — значит написать **один адаптер**, а не пересобирать бота.

---

## Получить доступ

| Платформа | Ссылка |
|----------|------|
| **Полный набор** | [Prediction-Markets-Trading-Bot-Toolkits](https://github.com/HarrierOnChain/Prediction-Markets-Trading-Bot-Toolkits) |
| **Telegram** | [@HarrierOnChain](https://t.me/HarrierOnChain) |
| **Обсуждения** | [GitHub Discussions](https://github.com/HarrierOnChain/Prediction-Markets-Trading-Bot-Toolkits/discussions) |

*Обычно отвечаем в течение нескольких часов.*

---

## Связанные площадки

[Polymarket](https://github.com/HarrierOnChain/Polymarket) · [Kalshi](https://github.com/HarrierOnChain/Kalshi) · [OG.com](https://github.com/HarrierOnChain/OG.com) · [Zeitgeist](https://github.com/HarrierOnChain/Zeitgeist)

> Полный каталог площадок — в [основном наборе →](https://github.com/HarrierOnChain/Prediction-Markets-Trading-Bot-Toolkits#venue-coverage)

---

## Отказ от ответственности

> Торговля на рынках предсказаний сопряжена с реальным финансовым риском. ПО предоставляется «как есть», без гарантий, и не является финансовой консультацией. Всегда тестируйте с `enable_trading: false` перед вводом реального капитала. Соблюдайте условия использования Drift BET и местное законодательство.

---

<div align="center">

**Торговый бот Drift BET · построен на движке [Prediction Market Toolkits](https://github.com/HarrierOnChain/Prediction-Markets-Trading-Bot-Toolkits)**

[![Telegram](https://img.shields.io/badge/Telegram-@HarrierOnChain-26A5E4?style=flat-square&logo=telegram)](https://t.me/HarrierOnChain)

</div>
