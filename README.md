# Hey LLM
**hey** is a command-line application that allows users to easily query a large language model locally, allowing users to avoid sending data to a LLM host server such as OpenAI, Microsoft, or Google.

[YouTube Video](https://youtu.be/cyfAuCjFWU8?si=ugiYMiaX7-9F_vVs)

## Setup
Run `./setup.sh` the project directory. This will download ollama if it's not already installed and will install the _llama2-uncensored_ LLM by default. The script can be modified to download anything in the [ollama library](https://ollama.com/library) instead of _llama2-uncensored_. If ollama has already been downloaded, it will set _llama2-uncensored_ as the default `OLLAMA_MODEL` that is used when making queries. This can be changed prior to script execution or post-script execution in the user's _.bashrc_ file. After that, the `hey-llm` binary gets copied over from the project directory to the user's _/usr/local/bin_ folder. Then, an alias is created so that the user can easily run the executable in the command line with the keyword `hey`.

## Usage
Once successfully set up, users can create a query simply by using the keyword `hey` and supplying the desired prompt in quotes.
```bash
$ hey "What's the US state where all of the movies are made?"
# California.
```

To use the previous chat context (for a follow-up question for example), users can add the `-r` flag before the prompt.
```bash
$ hey -r "Give me a state adjacent to that"
# A state that borders California is Nevada.
```
