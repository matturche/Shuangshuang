# 爽双 Shuangshuang

A small app to test yourself on pinyin tone pairs. You can check it over [there](https://shuangshuang.onrender.com/)!

## Context

One of the most daunting tasks for new Chinese learners is to master tones, especially for learners coming from a non-tonal language (such as English and French).
To overcome this, learners are encouraged to practice tone pairs, and online resources are available for this task, such as [this table](https://yoyochinese.com/chinese-learning-tools/tone-pairs), or [this test](https://www.dong-chinese.com/learn/sounds/pinyin/toneTrainer).
This project is similar to the latter: an audio recording of a tone pair plays and you have to guess it. It goes a little further, as you have a mode where you also have to input the full pinyin, and it is slightly more customizable.

## Building the project locally 

The project uses both [Rust](https://rust-lang.org/tools/install/) and [Python](https://www.python.org/about/gettingstarted/), be sure at least Rust is installed to run the website localy.

The project also uses Rust `nightly`, and requires that you've installed the `wasm` compilation target for your toolchain.

If you don't have Rust nightly, you can install it with
```sh
rustup toolchain install nightly --allow-downgrade
```

You can add the `wasm` compilation target to Rust using
```sh
rustup target add wasm32-unknown-unknown
```

The website is made with [Leptos](https://leptos.dev/) in Client Side Rendering (CSR).

You only need Python to rerun the preprocessing script in `py_scripts/`, this script cleans the `data/HSK2012_all.txt` file using the `data/in_HSK2012_all_missing-audio.txt` one.

If you wish you can set up the Python environment with [Poetry](https://python-poetry.org/), by navigating to the `py_scripts/` folder and using
```sh
poetry install
```

Then 
```sh
poetry shell
```

to start the environment, finally
```sh
python main.py
```

will run the script.

## Running the website

Once Rust is correctly set up, you can run

```sh
trunk serve --port 3000 --open
```

It will open your app in your default browser at `http://localhost:3000`.

## Resources

- Audio recordings, and data from [hugolpz/audio-cmn](https://github.com/hugolpz/audio-cmn)
- Pinyin entries from the [CC-CEDICT project](https://cc-cedict.org/wiki/), accessed through the [hanzipy](https://github.com/Synkied/hanzipy) library.

## Notes

The following hanzi pairs' pinyin were manually altered to match the audio recordings:

- 好处: from "hao3chu3" to "hao3chu"
- 因为: from "yin1wei4" to "yin1wei2"
