#!/bin/bash

if ! command -v ollama &> /dev/null
then
    curl -fsSL https://ollama.com/install.sh | sh
    ollama pull llama2-uncensored
    echo 'export OLLAMA_MODEL="llama2-uncensored"' >> ~/.bashrc
else
    echo 'export OLLAMA_MODEL="llama2-uncensored"' >> ~/.bashrc
fi

cp hey-llm /usr/local/bin/hey-llm
echo 'alias hey="/usr/local/bin/hey-llm $OLLAMA_MODEL"' >> ~/.bashrc